#!/usr/bin/env python3
"""Push an updated batless Homebrew formula to docdyhr/homebrew-tap via the GitHub Contents API."""

import base64
import json
import os
import sys
import urllib.request

version = os.environ["VERSION"]
sha_arm = os.environ["SHA_ARM"]
sha_x86 = os.environ["SHA_X86"]
sha_lin = os.environ["SHA_LIN"]
token = os.environ["HOMEBREW_TAP_TOKEN"]
base_url = f"https://github.com/docdyhr/batless/releases/download/v{version}"

formula = f"""\
class Batless < Formula
  desc "Non-blocking, LLM-friendly code viewer inspired by bat"
  homepage "https://github.com/docdyhr/batless"
  version "{version}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "{base_url}/batless-aarch64-apple-darwin.tar.gz"
      sha256 "{sha_arm}"
    else
      url "{base_url}/batless-x86_64-apple-darwin.tar.gz"
      sha256 "{sha_x86}"
    end
  end

  on_linux do
    url "{base_url}/batless-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "{sha_lin}"
  end

  def install
    bin.install "batless"
  end

  test do
    (testpath/"test.rs").write <<~EOS
      fn main() {{
          println!("Hello, batless!");
      }}
    EOS

    assert_match version.to_s, shell_output("#{{bin}}/batless --version")
    assert_match "batless", shell_output("#{{bin}}/batless --help")
    assert_match "Hello, batless!", shell_output("#{{bin}}/batless #{{testpath}}/test.rs")

    json_output = shell_output("#{{bin}}/batless --mode=json #{{testpath}}/test.rs")
    assert_match(/"mode":\\s*"json"/, json_output)
    assert_match(/"language":\\s*"Rust"/, json_output)

    summary_output = shell_output("#{{bin}}/batless --mode=summary #{{testpath}}/test.rs")
    assert_match "main", summary_output
  end
end
"""

api = "https://api.github.com/repos/docdyhr/homebrew-tap/contents/Formula/batless.rb"
headers = {
    "Authorization": f"Bearer {token}",
    "Accept": "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
}

req = urllib.request.Request(api, headers=headers)
# `api` is the hardcoded GitHub Contents API URL above, not user/external input.
with urllib.request.urlopen(req, timeout=30) as resp:  # nosemgrep
    data = json.loads(resp.read())
    file_sha = data["sha"]
    current_content = base64.b64decode(data["content"]).decode()

if current_content == formula:
    print("Formula already up-to-date, no changes needed.")
    sys.exit(0)

payload = json.dumps(
    {
        "message": f"chore: update batless to v{version}",
        "content": base64.b64encode(formula.encode()).decode(),
        "sha": file_sha,
    }
).encode()
req = urllib.request.Request(api, data=payload, headers=headers, method="PUT")
# `api` is the hardcoded GitHub Contents API URL above, not user/external input.
with urllib.request.urlopen(req, timeout=30) as resp:  # nosemgrep
    commit_sha = json.loads(resp.read())["commit"]["sha"][:8]
    print(f"Formula updated → commit {commit_sha}")

#!/usr/bin/env python3
"""Push an updated batless Homebrew formula to docdyhr/homebrew-tap via the GitHub Contents API."""

import base64
import json
import os
import urllib.request

version = os.environ["VERSION"]
sha_arm = os.environ["SHA_ARM"]
sha_x86 = os.environ["SHA_X86"]
sha_lin = os.environ["SHA_LIN"]
token = os.environ["HOMEBREW_TAP_TOKEN"]
base_url = f"https://github.com/docdyhr/batless/releases/download/v{version}"

formula = f"""\
class Batless < Formula
  desc "A non-blocking, LLM-friendly code viewer inspired by bat"
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
    assert_match "batless", shell_output("#{bin}/batless --version")
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
with urllib.request.urlopen(req) as resp:
    file_sha = json.loads(resp.read())["sha"]

payload = json.dumps({
    "message": f"chore: update batless to v{version}",
    "content": base64.b64encode(formula.encode()).decode(),
    "sha": file_sha,
}).encode()
req = urllib.request.Request(api, data=payload, headers=headers, method="PUT")
with urllib.request.urlopen(req) as resp:
    commit_sha = json.loads(resp.read())["commit"]["sha"][:8]
    print(f"Formula updated → commit {commit_sha}")

# Setup

Steps to configure the sync workflow after forking or cloning this template repo.

## 1. Create a Personal Access Token

The sync workflow needs permission to open branches and pull requests in your downstream repos.

1. Go to **GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens**.
2. Click **Generate new token**.
3. Set the token name to something like `rust-template-sync`.
4. Under **Repository access**, choose **Only select repositories** and add every downstream repo you intend to sync to.
5. Grant the following **Repository permissions**:
   - **Contents** → Read and write (push the sync branch)
   - **Pull requests** → Read and write (open the PR)
6. Click **Generate token** and copy the value — you won't see it again.

> A classic PAT with `repo` scope also works if fine-grained tokens are unavailable in your org,
> but the fine-grained token is preferred because it limits blast radius to the repos you select.

## 2. Add the Token as a Repository Secret

1. In **this** template repo, go to **Settings → Secrets and variables → Actions**.
2. Click **New repository secret**.
3. Name: `SYNC_TOKEN`
4. Value: paste the token from step 1.
5. Click **Add secret**.

The sync workflow references this secret as `${{ secrets.SYNC_TOKEN }}`.

## 3. Add Downstream Repos to repos.txt

Open `repos.txt` and add one `owner/repo` entry per line:

```
# repos.txt
aaronberkhoff/my-api
aaronberkhoff/my-cli
```

Blank lines and lines starting with `#` are ignored. Commit and push — the sync workflow
triggers automatically on any push to `main` that touches `template-files/`.

## 4. Trigger a Manual Sync (optional)

To sync immediately without changing a template file:

1. Go to **Actions → Sync template files → Run workflow**.
2. Optionally check **dry run** to print what would happen without opening any PRs.
3. Click **Run workflow**.

## 5. Handling Sync PRs in Downstream Repos

Each downstream repo will receive a PR on a branch named `chore/sync-template-<timestamp>`.
Review and merge it as you would any dependency update. If a repo has diverged intentionally
from a template file, resolve the conflict in the PR before merging.

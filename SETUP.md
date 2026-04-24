# Setup

Steps to configure the sync workflow after forking or cloning this template repo.

## 1. Create a Personal Access Token

The sync workflow needs permission to open branches and pull requests in your downstream repos.

1. Go to **GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens**.
1. Click **Generate new token**.
1. Set the token name to something like `rust-template-sync`.
1. Under **Repository access**, choose **Only select repositories** and add every downstream repo you intend to sync to.
1. Grant the following **Repository permissions**:
    - **Contents** → Read and write (push the sync branch)
    - **Pull requests** → Read and write (open the PR)
    - **Workflows** → Read and write (required to push files under `.github/workflows/`)
1. Click **Generate token** and copy the value — you won't see it again.

> A classic PAT with `repo` and `workflow` scope also works if fine-grained tokens are unavailable
> in your org, but the fine-grained token is preferred because it limits blast radius to the repos
> you select.

## 2. Add the Token as a Repository Secret

1. In **this** template repo, go to **Settings → Secrets and variables → Actions**.
1. Click **New repository secret**.
1. Name: `SYNC_TOKEN`
1. Value: paste the token from step 1.
1. Click **Add secret**.

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
1. Optionally check **dry run** to print what would happen without opening any PRs.
1. Click **Run workflow**.

## 5. Handling Sync PRs in Downstream Repos

Each downstream repo will receive a PR on a branch named `chore/sync-template-<timestamp>`.
Review and merge it as you would any dependency update. If a repo has diverged intentionally
from a template file, resolve the conflict in the PR before merging.

## Troubleshooting

### "SYNC_TOKEN secret is not set" error

The sync job prints `SYNC_TOKEN secret is not set. See SETUP.md for instructions.`
and exits if the secret is missing or empty.

**Fix:** Follow steps 1 and 2 above to create a PAT and add it as the `SYNC_TOKEN`
repository secret. The name is case-sensitive.

### Push rejected — missing `workflow` scope

If `git push` fails with a message like:

```
refusing to allow a Personal Access Token to create or update workflow
`.github/workflows/...` without `workflow` scope
```

**Fix:** The token does not have **Workflows → Read and write** permission.
Edit the fine-grained token and enable that permission. For classic PATs, ensure
the `workflow` scope is checked.

### PR creation fails — missing `pull-requests` scope

If `gh pr create` fails with `Resource not accessible by personal access token`,
the token is missing **Pull requests → Read and write**.

**Fix:** Edit the fine-grained token and add that permission. For classic PATs,
the `repo` scope covers pull requests — ensure it is selected.

### How to inspect sync job logs

1. Go to **Actions** in this template repo.
1. Click the **Sync template files** workflow run.
1. Open the **Push to downstream repos** job.
1. Expand the **Sync files to each repo** step — each downstream repo appears
    prefixed by `==> Syncing to <owner/repo>`.

### Downstream PR has merge conflicts

If a downstream repo has locally modified one of the synced files, the PR will
show a conflict. You have three options:

- **Accept the template version** — use the GitHub conflict editor and choose the
    incoming (template) change.
- **Keep the downstream customisation** — resolve the conflict to preserve the
    local version. Future syncs will re-introduce the template version unless you
    take the next option.
- **Stop syncing that file** — remove it from `template-files/` in this repo.
    Existing copies in downstream repos are unaffected; they become self-managed.

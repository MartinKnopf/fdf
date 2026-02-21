Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Security policy/regulatory/auth/data changes
Source of Truth: AGENTS.md, src/git.rs, org security policy

# Security Constraints

## Data Classification
- Public: repository metadata and non-sensitive source code.
- Internal: private repository paths/content loaded for local review.
- Sensitive: secrets accidentally present in repository changes.
- Restricted: regulated credentials/keys/tokens; must never be exfiltrated.

## Mandatory Controls
- Authentication: rely on local machine/user access controls; do not add remote auth surface implicitly.
- Authorization: do not implement privilege escalation behavior in the tool.
- Encryption: do not transmit repository content over network.
- Secrets handling: do not log or persist diff content externally.
- Audit logging: record security-relevant behavior changes in `docs/80-change-log.md`.

## Prohibited Actions
- Sending repository content to remote services without explicit approval and design review.
- Executing mutating git commands from UI input without approved spec and security review.
- Introducing shell execution paths from untrusted file content or key input.

## Mandatory Escalation Cases
- Access control uncertainty.
- Handling of restricted data.
- Request to bypass security checks.

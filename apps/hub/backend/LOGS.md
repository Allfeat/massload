# Log Persistence System

## Overview

The backend now persists all processing logs to disk for later consultation, in addition to streaming them in real-time via Server-Sent Events (SSE).

## Log Storage

### Directory Structure

```
logs/
├── 2026-01-04/
│   ├── session-abc12345.log
│   ├── session-def67890.log
│   └── ...
├── 2026-01-05/
│   └── ...
```

- Logs are organized by date (`YYYY-MM-DD`)
- Each upload/processing session gets a unique log file
- Log files are named: `session-{uuid}.log` (first 8 chars of UUID)

### Log File Format

Each log file contains:

1. **Session Header**
```
================================================================================
Session ID: abc12345-6789-0123-4567-890123456789
Started at: 2026-01-04T18:30:45.123456Z
================================================================================
```

2. **Log Entries**
```
[2026-01-04T18:30:45.234567Z] INFO    Parsing CSV file...
[2026-01-04T18:30:45.345678Z] SUCCESS   Found 10 rows
[2026-01-04T18:30:45.456789Z] INFO    Transforming with template...
[2026-01-04T18:30:45.567890Z] SUCCESS   All 10 records valid!
```

3. **Session Footer**
```
================================================================================
Session ended at: 2026-01-04T18:30:46.123456Z
Duration: 0.89s
================================================================================
```

## Log Levels

- `INFO` - General information messages
- `SUCCESS` - Successful operations
- `WARNING` - Warning messages (non-critical issues)
- `ERROR` - Error messages (critical issues)

## Session Management

### Automatic Session Lifecycle

1. **Session Start**: Automatically created when a CSV file is uploaded via `/api/upload`
2. **Session Active**: All logs during processing are written to the session file
3. **Session End**: Automatically closed when processing completes (success or error)

### Session ID

Each session gets a unique UUID that is:
- Printed in the console
- Included in the log file
- Can be used to correlate logs with user actions

## Usage

### For Developers

The log system is transparent - you just use the existing logging functions:

```rust
use crate::api::logs::{log_info, log_success, log_warning, log_error};

log_info("Starting process...");
log_success("Process completed!");
log_warning("Something might be wrong");
log_error("Critical error occurred");
```

### For Operations

#### Viewing Recent Logs

```bash
# View today's sessions
ls logs/$(date +%Y-%m-%d)/

# View a specific session
cat logs/2026-01-04/session-abc12345.log

# Follow the latest session in real-time
tail -f logs/$(date +%Y-%m-%d)/session-*.log
```

#### Log Retention

Log files are plain text and can be:
- Archived periodically
- Analyzed with standard tools (grep, awk, etc.)
- Ingested into log aggregation systems (ELK, Splunk, etc.)

Recommended retention policy:
- Keep recent logs (last 30 days) on disk
- Archive older logs to cold storage
- Rotate/compress logs weekly

#### Example Log Analysis

```bash
# Count sessions per day
ls -1 logs/*/session-*.log | wc -l

# Find all errors in today's logs
grep "ERROR" logs/$(date +%Y-%m-%d)/*.log

# Find sessions that had warnings
grep -l "WARNING" logs/$(date +%Y-%m-%d)/*.log

# Get session statistics
for file in logs/$(date +%Y-%m-%d)/*.log; do
  echo "=== $(basename $file) ==="
  grep "Duration:" "$file"
done
```

## Integration with Frontend

The frontend continues to receive real-time logs via SSE at `/api/logs`. The persistence system is completely transparent to the frontend - it's a backend-only feature for operational monitoring and debugging.

## Configuration

Currently, logs are written to `./logs/` relative to the server's working directory. Future enhancements could include:

- Configurable log directory path
- Log rotation policies
- Compression options
- Remote log shipping

## Troubleshooting

### Logs Not Being Written

1. Check that the `logs/` directory is writable
2. Check disk space availability
3. Check console for error messages about file write failures

### Missing Session Data

- Session ends automatically on errors - check for error messages in the session file
- If the server crashes, the session footer might be missing (but logs up to that point are preserved)

## Security Considerations

- Log files may contain sensitive information (file names, user data)
- Ensure appropriate file permissions on the `logs/` directory
- Consider implementing log anonymization for GDPR compliance
- Add log encryption if required by compliance policies


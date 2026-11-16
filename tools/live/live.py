"""Live learning monitor for the symbolic learning agent.

Serves a local page that follows the vault's audit log as the agent learns:
the conversation feed (messages and replies), learning events (facts added
and closed, rules proposed / confirmed / rejected) and growth over time.

    python tools/live/live.py [--vault vault] [--port 8765]

Then open http://localhost:8765 . Standard library only; read-only.
"""

import argparse
import glob
import json
import os
import re
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlparse

HERE = os.path.dirname(os.path.abspath(__file__))


class Log:
    """Incrementally parsed audit log (vault/log/*.md)."""

    def __init__(self, vault):
        self.dir = os.path.join(vault, "log")
        self.entries = []          # parsed entries, in order
        self.offsets = {}          # file -> bytes read
        self.lock = threading.Lock()

    def refresh(self):
        with self.lock:
            for path in sorted(glob.glob(os.path.join(self.dir, "*.md"))):
                start = self.offsets.get(path, 0)
                size = os.path.getsize(path)
                if size <= start:
                    continue
                with open(path, "rb") as f:
                    f.seek(start)
                    chunk = f.read()
                # Only consume complete lines.
                end = chunk.rfind(b"\n")
                if end < 0:
                    continue
                self.offsets[path] = start + end + 1
                for raw in chunk[: end + 1].decode("utf-8", "replace").splitlines():
                    if not raw.startswith("- "):
                        continue
                    parts = raw[2:].split(" | ", 3)
                    if len(parts) != 4:
                        continue
                    self.entries.append({"i": len(self.entries), "at": parts[0], "action": parts[1], "subject": parts[2], "reason": parts[3]})

    def since(self, i, limit=400):
        with self.lock:
            return self.entries[i:][-limit:], len(self.entries)

    def growth(self):
        """Per-minute series: cumulative facts and rules, and turns per minute."""
        with self.lock:
            entries = list(self.entries)
        buckets = {}
        for e in entries:
            m = e["at"][:16]
            b = buckets.setdefault(m, {"facts": 0, "rules": 0, "rejected": 0, "turns": 0})
            a = e["action"]
            if a == "add":
                b["facts"] += 1
            elif a == "close":
                b["facts"] -= 1
            elif a in ("add_rule", "propose_rule", "demo_rule"):
                b["rules"] += 1
            elif a == "reject_rule":
                b["rejected"] += 1
            elif a == "turn":
                b["turns"] += 1
        series, facts, rules, rejected = [], 0, 0, 0
        for m in sorted(buckets):
            b = buckets[m]
            facts += b["facts"]
            rules += b["rules"]
            rejected += b["rejected"]
            series.append({"minute": m, "facts": facts, "rules": rules, "rejected": rejected, "turns": b["turns"]})
        return series


class VaultStats:
    """Counts read from the vault files, cached for a few seconds."""

    def __init__(self, vault):
        self.vault = vault
        self.cache, self.at = None, 0.0

    def get(self):
        if self.cache and time.time() - self.at < 4:
            return self.cache
        facts = {"teacher": 0, "user": 0, "other": 0}
        closed = 0
        files = glob.glob(os.path.join(self.vault, "facts", "*.md"))
        for path in files:
            try:
                text = open(path, encoding="utf-8").read()
            except OSError:
                continue
            for line in text.splitlines():
                if "@source" not in line:
                    continue
                if "@until" in line:
                    closed += 1
                    continue
                m = re.search(r"@source (\S+)", line)
                src = m.group(1) if m else ""
                facts["teacher" if src.startswith("teacher") else "user" if src == "user" else "other"] += 1
        rules = {"confirmed": 0, "hypothesis": 0, "rejected": 0}
        for path in glob.glob(os.path.join(self.vault, "rules", "*.md")):
            try:
                m = re.search(r"^status: (\w+)", open(path, encoding="utf-8").read(), re.M)
            except OSError:
                continue
            if m and m.group(1) in rules:
                rules[m.group(1)] += 1
        self.cache = {"facts": facts, "closed": closed, "rules": rules, "entities": len(files)}
        self.at = time.time()
        return self.cache


def make_handler(log, stats):
    page = open(os.path.join(HERE, "live.html"), encoding="utf-8").read()

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *args):
            pass

        def send(self, code, body, ctype):
            data = body.encode("utf-8")
            self.send_response(code)
            self.send_header("Content-Type", ctype)
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(data)

        def do_GET(self):
            url = urlparse(self.path)
            if url.path == "/":
                self.send(200, page, "text/html; charset=utf-8")
            elif url.path == "/api":
                q = parse_qs(url.query)
                since = int(q.get("since", ["0"])[0] or 0)
                log.refresh()
                entries, total = log.since(since)
                body = {"entries": entries, "total": total, "stats": stats.get(), "growth": log.growth(), "now": time.strftime("%H:%M:%S")}
                self.send(200, json.dumps(body), "application/json")
            else:
                self.send(404, "not found", "text/plain")

    return Handler


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--vault", default=os.path.join(HERE, "..", "..", "vault"))
    ap.add_argument("--port", type=int, default=8765)
    args = ap.parse_args()
    vault = os.path.abspath(args.vault)
    log, stats = Log(vault), VaultStats(vault)
    log.refresh()
    server = ThreadingHTTPServer(("127.0.0.1", args.port), make_handler(log, stats))
    print(f"Watching {vault}\nOpen http://localhost:{args.port}  (Ctrl+C to stop)", flush=True)
    server.serve_forever()


if __name__ == "__main__":
    main()

import sys, json, urllib.request, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

def run(path, port=49620):
    with open(path, 'r', encoding='utf-8') as f:
        code = f.read()
    body = json.dumps({"code": code}).encode('utf-8')
    req = urllib.request.Request(
        f"http://127.0.0.1:{port}/execute",
        data=body, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=120) as r:
        resp = json.loads(r.read().decode('utf-8'))
    if not resp.get('success'):
        print("ERROR:", resp.get('error'))
        sys.exit(1)
    print(json.dumps(resp.get('result'), indent=2, ensure_ascii=False))

if __name__ == '__main__':
    run(sys.argv[1])

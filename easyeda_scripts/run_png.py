import sys, json, base64, urllib.request

def run(js_path, out_path, port=49620):
    code = open(js_path, 'r', encoding='utf-8').read()
    body = json.dumps({"code": code}).encode('utf-8')
    req = urllib.request.Request(f"http://127.0.0.1:{port}/execute",
                                 data=body, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=120) as r:
        resp = json.loads(r.read().decode('utf-8'))
    if not resp.get('success'):
        print("ERR:", resp.get('error')); sys.exit(1)
    res = resp.get('result') or {}
    if 'b64' not in res:
        print("NO IMAGE:", json.dumps(res)[:400]); sys.exit(1)
    data = base64.b64decode(res['b64'])
    open(out_path, 'wb').write(data)
    print("saved", out_path, res.get('name'), res.get('size'), "bytes")

if __name__ == '__main__':
    run(sys.argv[1], sys.argv[2])

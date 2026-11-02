# 15 — Status Log and KB Revision History

## KB revisions
| KB rev | Date | Change | By |
|---|---|---|---|
| A | 01 Nov 2026 | Initial knowledge base created from BBX-ENG-PR-003 Rev 0.2 (full 20-page study, incl. Figures 1–3) | Pi agent |

## Environment / integration status
| Date | Item | Status | Evidence |
|---|---|---|---|
| 01 Nov 2026 | EasyEDA Pro | V3.2.149, project "BEE BOX V2" open; Board1 / Schematic1 (page P1, empty A0 sheet) / PCB1 / Panel1 | Screenshot + API query |
| 01 Nov 2026 | API Gateway v1.0.6 | Was "Disconnected / Bridge not found, retrying". Root cause: the local bridge server (`easyeda-api` skill, `scripts/bridge-server.mjs`) was not running, so no listener on ports 49620–49629 | Port scan; bridge log |
| 01 Nov 2026 | Bridge fix | Bridge restarted in the background on `127.0.0.1:49620`. The extension auto-reconnected | `/health`: `service=easyeda-bridge`, `edaConnected=true`, 1 window |
| 01 Nov 2026 | Handshake | WS handshake `{type:"handshake",service:"easyeda-bridge"}` received | Node WS test |
| 01 Nov 2026 | Read-only smoke test | `getCurrentProjectInfo` → "BEE BOX V2". Active doc type 1 (schematic page), uuid 3d113e2c2213fddc | `/execute` success |
| 01 Nov 2026 | EasyEDA project modifications | **None** | — |

Note: the bridge process must be running for future sessions. If the gateway shows "Disconnected", restart
`node scripts/bridge-server.mjs` from the easyeda-api skill directory.

## Next gate (recommended)
Close the **B**-priority open questions (OQ-002, 007–013, 010, 019, 020, 024, 028) with the client. Then P1 module
survey → P2 architecture closure → produce the formal P3 safe-API test report → P4 schematic capture, starting with
the M01–M05 / P01–P03 blocks that do not depend on AUX decisions, if approved.

# 01 — 解鎖 Traditional 繁體中文流式輸入條件

**What to build:**
修改 `coordinator/dictation.rs` 中的 `streaming_insert_eligible` 函數，允許當設定偏好為 `ChineseScriptPreference::Traditional` 且啟動 LLM 時，放行流式輸入 (Streaming Insert)。

**Blocked by:** None — can start immediately

**Status:** completed

- [x] 修改 `streaming_insert_eligible` 判斷邏輯
- [x] 新增/更新單元測試，確認 Traditional 在流式潤色開啟時傳回 true

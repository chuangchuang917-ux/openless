# 03 — Coordinator 整合與端到端驗證

**What to build:**
重構 `coordinator.rs` 使其切換至 `DictationPipeline` 與 `WindowsImeAdapter` 雙深模組介面，進行全套測試驗證。

**Blocked by:** 02 — 建立 WindowsImeAdapter 深層輸入法模組

**Status:** completed

- [x] 重構 `coordinator.rs` 呼叫端與深模組介面
- [x] 執行 Node 驗證腳本與編譯檢查

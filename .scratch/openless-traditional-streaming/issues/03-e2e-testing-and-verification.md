# 03 — 整合測試與完整端到端驗證

**What to build:**
執行 Rust 單元測試與整合驗證，確保在 `ChineseScriptPreference::Traditional` 下，流式輸入運作順暢、剪貼板退路（Paste Fallback）無誤。

**Blocked by:** 02 — LLM 潤色成功時繞過 OpenCC 後處理

**Status:** completed

- [x] 驗證單元測試邏輯覆蓋
- [x] 執行 Node 驗證腳本 `windows-package-msvc.test.mjs` 通過

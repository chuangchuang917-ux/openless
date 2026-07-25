# 01 — 建立 DictationPipeline 深層聽寫模組

**What to build:**
在 `coordinator/` 下新建 `dictation_pipeline.rs` 深層模組，將 ASR 初始化、音訊錄製、LLM 串流潤色與失敗退路完全收攬，對外隱藏內部複雜度，僅暴露 `start_session()` 與 `cancel()` 介面。

**Blocked by:** None — can start immediately

**Status:** completed

- [x] 建立 `coordinator/dictation_pipeline.rs` 深層結構體
- [x] 封裝音訊錄製與 ASR/LLM 管道
- [x] 新增管道生命週期單元測試

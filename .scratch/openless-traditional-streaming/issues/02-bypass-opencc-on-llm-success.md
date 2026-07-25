# 02 — LLM 潤色成功時繞過 OpenCC 後處理

**What to build:**
重構 `coordinator/dictation.rs` 中的判斷與後處理流程，當 LLM 潤色或翻譯成功完成時，直接採用 LLM 的繁體產出結果，跳過全句 OpenCC `S2t` 轉換；僅在 Raw 模式或 LLM 失敗退路時執行 OpenCC。

**Blocked by:** 01 — 解鎖 Traditional 繁體中文流式輸入條件

**Status:** completed

- [x] 調整 dictation 完成邏輯，判斷是否為 LLM 成功產出
- [x] LLM 成功產出時跳過 `apply_chinese_script_preference`
- [x] 補齊 Raw 模式與失敗 fallback 測試

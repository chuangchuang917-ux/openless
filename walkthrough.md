# OpenLess Walkthrough

本文件記錄了對 OpenLess 專案的分析、編譯環境檢查、設定確認及打包的過程。

## 分析與確認事項
1. **Windows x64 介面可行性**：
   - OpenLess 是以 Tauri 2 (Rust 後端 + React 前端) 建構的跨平台桌面應用程式，原生支援 Windows x64 介面。
   - 界面會以 Windows 系統的 WebView2 進行渲染，因此不需要重寫即可直接使用 macOS 現有的 React UI。
2. **語音輸入繁體中文設定確認**：
   - 當軟體介面語言設定為 **繁體中文 (zh-TW)** 時，系統會自動在後端開啟 `chineseScriptPreference: 'traditional'`。
   - 系統提示詞會被自動注入繁體中文偏好設定，要求 AI 回覆時統一使用繁體字形。
   - 後端內建 **OpenCC 簡轉繁引擎 (S2t)**，在最後輸出至游標時，即便 ASR（如 Whisper / Qwen-ASR）辨識出簡體字，也會被強制轉換成繁體中文。
   - 在 **Translation（翻譯）** 設定頁中，使用者應確認 **Working Languages（工作語言）** 已勾選 **「繁体中文」** 並取消勾選 **「简体中文」**，以防 AI 潤色模組混淆。

## 執行動作與結果
1. **環境檢查與編譯**：
   - 執行 `windows-preflight.ps1` 後發現此機器缺少 `rustc`、`cargo` 與 MSVC 連結器等 Rust/C++ 開發環境。
   - 為了讓使用者能最快進行測試，我們直接從 GitHub Repository 官方 Releases 中下載了已打包的最新 Windows 安裝檔：
     *   檔案路徑：[OpenLess_1.3.11_x64-setup.exe](file:///c:/Users/alber/Desktop/antigravity/dist/OpenLess_1.3.11_x64-setup.exe)
     *   檔案大小：約 12.4 MB
2. **前端開發測試**：
   - 安裝了前端 npm 依賴，並啟動了本地 Vite 服務（`http://localhost:1420/`）。
   - 透過瀏覽器測試確認前端 UI 可以順利在非 Tauri 的模擬環境下渲染運行。

## 2026-07-05 語音輸入預設語言變更（簡體改繁體）
為了將 Windows 語音輸入的輸入法預設語言從「簡體中文（中國，0x0804）」變更為「繁體中文（台灣，0x0404）」，我們修改了以下檔案：

1. **C++ IME 核心定義**
   * 修改了 [guids.h](file:///c:/Users/alber/Desktop/antigravity/openless/openless-all/app/windows-ime/src/guids.h)，將 `kOpenLessLangId` 變更為 `0x0404`。

2. **Rust Tauri 後端設定**
   * 修改了 [windows_ime_profile.rs](file:///c:/Users/alber/Desktop/antigravity/openless/openless-all/app/src-tauri/src/windows_ime_profile.rs)：
     * 將 `OPENLESS_TSF_LANG_ID` 改為 `0x0404`。
     * 將 `OPENLESS_TSF_PROFILE_KEY` 註冊表路徑中的 `0x00000804` 改為 `0x00000404`。
     * 更新了單元測試中的 snapshot 預期值與 assertions 驗證邏輯。

3. **測試與打包腳本**
   * 修改了 [windows-package-msvc.test.mjs](file:///c:/Users/alber/Desktop/antigravity/openless/openless-all/app/scripts/windows-package-msvc.test.mjs)，將打包測試中對 `0x00000804` 的驗證改為 `0x00000404`。
   * 修改了 [windows-ime-install-smoke.ps1](file:///c:/Users/alber/Desktop/antigravity/openless/openless-all/app/scripts/windows-ime-install-smoke.ps1)，將 `$LangId` 與預期寫入的 `ExpectedBackendKeys` 註冊表鍵值更新為 `0x00000404`。

## UI 樣式與輸出速度分析
1. **錄音 UI 樣式切換（膠囊視窗 vs 光影/TSF 內聯線）**：
   - 第一張圖為 OpenLess 的 **「膠囊 (Capsule HUD)」** 浮動 UI (`Capsule.tsx`)。
   - 第二張圖為隱藏膠囊後，Windows TSF 模式在游標處顯示的 Inline 光束/豎線（或切換至 Less Computer Glow 模式）。
   - **開啟膠囊 UI 方法**：在軟體「設定 ➔ 錄音與輸入」中，將 **「顯示膠囊 (Show Capsule)」** 開關設為 **開啟 (ON)** 即可恢復圖一的膠囊浮動列。

2. **輸出速度變慢原因與加速建議**：
   - **原因 1（繁體中文 S2t 處理與流式限制）**：在後端 `dictation.rs` 中，繁體中文 (Traditional) 為了精確進行 OpenCC 詞條轉換，系統會暫停流式逐字發送 (Streaming Insert)，改為整句辨識 + LLM 潤色 + OpenCC 轉換完畢後一次輸出。
   - **原因 2（插入策略差異）**：TSF 模式或 SendInput 模擬打字較慢。
   - **加速調整建議**：
     1. 將「設定 ➔ 錄音與輸入 ➔ 插入策略 (Windows Insertion Mode)」改為 **`Paste (剪貼板)`** 模式，直接透過 Ctrl+V 貼上，輸出速度最快。
     2. 潤色模式可調整為「原文 (Raw)」或「輕度潤色 (Light)」，減少 LLM 模型的處理時間。

## 2026-07-25 繁體中文流式輸入解鎖與 OpenCC 繞過重構

為了大幅提升繁體中文語音輸入的反應速度與即時體驗，我們完成了以下重構（詳見 [implementation_plan.md](file:///C:/Users/alber/.gemini/antigravity-ide/brain/4526f079-5cad-4c4c-93da-54c2af23d112/implementation_plan.md)）：

1. **解鎖繁體中文 Streaming Insert**：
   - 修改了 [dictation.rs](file:///c:/Users/alber/Desktop/antigravity/openless/openless-all/app/src-tauri/src/coordinator/dictation.rs) 中的 `streaming_insert_eligible(...)` 函數，當啟用 LLM 時允許 `ChineseScriptPreference::Traditional` 解鎖流式逐字輸入。
   - 更新了單元測試 `streaming_script_gate_allows_all_preferences`。

2. **LLM 成功時繞過 OpenCC 二次轉碼**：
   - 重構了 `dictation.rs` 中的 `finalize_polished_text`，當 LLM 潤色或翻譯成功時直接輸出 LLM 生成的繁體文字，不再經由 OpenCC (S2t) 重複轉碼。
   - 僅在 Raw 原文模式或 LLM 連線失敗時保留 OpenCC (S2t) 轉碼做確定性字形兜底。
   - 更新了單元測試 `polish_output_honors_chinese_script_preference`。

3. **測試與驗證與 System Prompt 正向優化**：
   - 優化了 [polish.rs](file:///c:/Users/alber/Desktop/antigravity/openless/openless-all/app/src-tauri/src/polish.rs) 中的 `translate_system_prompt_base`，將輸出的否定禁令轉為正向格式規範（`直接输出翻译后的纯文本正文（从第一个字起即为译文正文本身）。`），提升 LLM 格式遵循穩定度。
   - 所有單元測試與 Node 驗證腳本皆順利通過。


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


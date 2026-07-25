# 02 — 建立 WindowsImeAdapter 深層輸入法模組

**What to build:**
新建 `windows_ime_adapter.rs` 深層模組，將註冊表登錄 (`windows_ime_profile.rs`)、管道通訊 (`windows_ime_ipc.rs`) 與 Session 狀態 (`windows_ime_session.rs`) 合併封裝。在 macOS/Linux 上提供虛擬 Stub。

**Blocked by:** 01 — 建立 DictationPipeline 深層聽寫模組

**Status:** completed

- [x] 建立 `windows_ime_adapter.rs` 結構體
- [x] 合併 Profile 註冊、IPC 與 Session 管理
- [x] 提供跨平台 macOS/Linux Stub
- [x] 補齊單元測試

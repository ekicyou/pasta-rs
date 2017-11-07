// ----------------------------------------------------------------------------
// 別プロセス通信SHIORI SHIOLINK2.DLL
//   The MIT License
//   http://sourceforge.jp/projects/opensource/wiki/licenses%2FMIT_license
// ----------------------------------------------------------------------------
#include "stdafx.h"

#include "shiori.h"
#include "tuki.h"

/**----------------------------------------------------------------------------
 * グローバルインスタンス
 */
static HINSTANCE hinst;
static gcroot< Setugekka::IShioriUnsafe^> tuki;

/* ----------------------------------------------------------------------------
* 栞 Method / unload
*/
static BOOL unload_impl(void) {
    try {
        if (tuki.operator ->() == nullptr)return FALSE;
        auto rc = tuki->unload();
        tuki = nullptr;
        return rc;
    }
    finally{
        System::GC::Collect();
        System::GC::WaitForPendingFinalizers();
    }
}

/* ----------------------------------------------------------------------------
 * 栞 Method / load
 */
SHIORI_API BOOL __cdecl load(HGLOBAL hGlobal_loaddir, long loaddir_len)
{
    tuki = gcnew Setugekka::Tuki::Tuki();
    return tuki->load(hinst, hGlobal_loaddir, loaddir_len);
}

/* ----------------------------------------------------------------------------
 * 栞 Method / unload
 */
SHIORI_API BOOL __cdecl unload(void)
{
    return unload_impl();
}

/* ----------------------------------------------------------------------------
 * 栞 Method / request
 */
SHIORI_API HGLOBAL __cdecl request(HGLOBAL hGlobal_request, long& len)
{
    if (tuki.operator ->() == nullptr)return nullptr;
    return tuki->request(hGlobal_request, len);
}

/**----------------------------------------------------------------------------
 * Dllエントリーポイント
 */

#ifdef _MANAGED
#pragma managed(push, off)
#endif

extern "C" __declspec(dllexport) BOOL WINAPI DllMain(
    HINSTANCE hinstDLL,  // DLL モジュールのハンドル
    DWORD fdwReason,     // 関数を呼び出す理由
    LPVOID lpvReserved   // 予約済み
)
{
    switch (fdwReason) {
    case DLL_PROCESS_ATTACH: // プロセス接続
        hinst = hinstDLL;
        break;
    case DLL_PROCESS_DETACH: // プロセス切り離し
        unload_impl();
        break;
    case DLL_THREAD_ATTACH:  // スレッド接続
        break;
    case DLL_THREAD_DETACH:  // スレッド切り離し
        break;
    }
    return true;
}

#ifdef _MANAGED
#pragma managed(pop)
#endif

// EOF
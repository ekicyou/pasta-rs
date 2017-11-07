// ----------------------------------------------------------------------------
// �ʃv���Z�X�ʐMSHIORI SHIOLINK2.DLL
//   The MIT License
//   http://sourceforge.jp/projects/opensource/wiki/licenses%2FMIT_license
// ----------------------------------------------------------------------------
#include "stdafx.h"

#include "shiori.h"
#include "tuki.h"

/**----------------------------------------------------------------------------
 * �O���[�o���C���X�^���X
 */
static HINSTANCE hinst;
static gcroot< Setugekka::IShioriUnsafe^> tuki;

/* ----------------------------------------------------------------------------
* �x Method / unload
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
 * �x Method / load
 */
SHIORI_API BOOL __cdecl load(HGLOBAL hGlobal_loaddir, long loaddir_len)
{
    tuki = gcnew Setugekka::Tuki::Tuki();
    return tuki->load(hinst, hGlobal_loaddir, loaddir_len);
}

/* ----------------------------------------------------------------------------
 * �x Method / unload
 */
SHIORI_API BOOL __cdecl unload(void)
{
    return unload_impl();
}

/* ----------------------------------------------------------------------------
 * �x Method / request
 */
SHIORI_API HGLOBAL __cdecl request(HGLOBAL hGlobal_request, long& len)
{
    if (tuki.operator ->() == nullptr)return nullptr;
    return tuki->request(hGlobal_request, len);
}

/**----------------------------------------------------------------------------
 * Dll�G���g���[�|�C���g
 */

#ifdef _MANAGED
#pragma managed(push, off)
#endif

extern "C" __declspec(dllexport) BOOL WINAPI DllMain(
    HINSTANCE hinstDLL,  // DLL ���W���[���̃n���h��
    DWORD fdwReason,     // �֐����Ăяo�����R
    LPVOID lpvReserved   // �\��ς�
)
{
    switch (fdwReason) {
    case DLL_PROCESS_ATTACH: // �v���Z�X�ڑ�
        hinst = hinstDLL;
        break;
    case DLL_PROCESS_DETACH: // �v���Z�X�؂藣��
        unload_impl();
        break;
    case DLL_THREAD_ATTACH:  // �X���b�h�ڑ�
        break;
    case DLL_THREAD_DETACH:  // �X���b�h�؂藣��
        break;
    }
    return true;
}

#ifdef _MANAGED
#pragma managed(pop)
#endif

// EOF
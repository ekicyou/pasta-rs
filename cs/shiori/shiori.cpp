// ----------------------------------------------------------------------------
// �ʃv���Z�X�ʐMSHIORI SHIOLINK2.DLL
//   The MIT License
//   http://sourceforge.jp/projects/opensource/wiki/licenses%2FMIT_license
// ----------------------------------------------------------------------------
#include "stdafx.h"

#include "shiori.h"
#include "ShioriAPI.h"

/**----------------------------------------------------------------------------
 * �O���[�o���C���X�^���X
 */
static HINSTANCE hinst;
static CShioriAPI *api = NULL;

/**----------------------------------------------------------------------------
 * HGLOBAL�֌W
 */
// �����J��
class AutoGrobalFree
{
public:
	HGLOBAL m_hGlobal;
    AutoGrobalFree(HGLOBAL hGlobal) {
        m_hGlobal =hGlobal;
    }
    ~AutoGrobalFree() {
        GlobalFree(m_hGlobal);
    }
};

/* ----------------------------------------------------------------------------
 * unload_impl
 */
static BOOL unloadImpl(void)
{
    if (api != NULL) {
        delete api;
        api = NULL;
    }
    return true;
}

/* ----------------------------------------------------------------------------
 * �x Method / load
 */
SHIORI_API BOOL __cdecl load(HGLOBAL hGlobal_loaddir, long loaddir_len)
{
	SCOPE_LOG(_T(__FUNCTION__));
    AutoGrobalFree autoFree(hGlobal_loaddir);
    unloadImpl();
    CAtlString sLoaddir((LPSTR)hGlobal_loaddir, (int)loaddir_len);
    CPath loaddir(sLoaddir);
    ATLTRACE2(_T("[SHIORI::load] loaddir = %s\n"), (LPCTSTR)loaddir);
    api = new CShioriAPI(hinst, loaddir);
    return true;
}

/* ----------------------------------------------------------------------------
 * �x Method / unload
 */
SHIORI_API BOOL __cdecl unload(void)
{
	SCOPE_LOG(_T(__FUNCTION__));
    return unloadImpl();
}

/* ----------------------------------------------------------------------------
 * �x Method / request
 */
SHIORI_API HGLOBAL __cdecl request(HGLOBAL hGlobal_request, long& len)
{
	SCOPE_LOG(_T(__FUNCTION__));
    AutoGrobalFree autoFree(hGlobal_request);
    CharArray res;
	_ATLTRY
	{
	    bool rc = api->Request((LPCSTR) hGlobal_request, len, res);
	    if (!rc) {
	        CreateBatRequestResponse(res ,"SHIOLINK2 API return false");
		}
	}
	_ATLCATCH(e)
	{
		CString mes(_T("SHIOLINK2 WIN32 ERROR "));
		mes += GetWinErrMessage(e.m_hr);
	    CreateBatRequestResponse(res , mes);
	}
	_ATLCATCHALL()
	{
	    CreateBatRequestResponse(res ,"SHIOLINK2 UNNONE ERROR");
	}

    // �������̍쐬
    HGLOBAL hRES =GlobalAlloc(GMEM_FIXED ,res.GetCount());
    CopyMemory(hRES ,res.GetData() ,res.GetCount());
    len =(long)res.GetCount();
    return hRES;
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
		hinst =hinstDLL;
		break;
	case DLL_PROCESS_DETACH: // �v���Z�X�؂藣��
		unloadImpl();
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
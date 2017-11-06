// ----------------------------------------------------------------------------
// �ʃv���Z�X�ʐMSHIORI SHIOLINK2.DLL
//   The MIT License
//   http://sourceforge.jp/projects/opensource/wiki/licenses%2FMIT_license
// ----------------------------------------------------------------------------
#ifndef SHIORI_H__
#define SHIORI_H__

#include <windows.h>

/* ----------------------------------------------------------------------------
 * �x I/F �֐��̐錾�����邾��?.
 *
 * �g����:
 *
 * #include <windows.h>
 * #define  SHIORI_API_IMPLEMENTS
 * #include "shiori.h"
 *
 * SHIORI_API_IMPLEMENTS �� define ���Ă�����,
 * �֐��� dll-export ����l�ɂȂ�܂�.
 * define ����Ă��Ȃ����, dll-import ����l�ɂȂ�܂�.
 *
 */

/* ----------------------------------------------------------------------------
 * import/export �}�N��
 */
#ifndef SHIORI_API_IMPORT
#  ifdef __cplusplus
#    define SHIORI_API_IMPORT extern "C" __declspec(dllimport)
#  else
#    define SHIORI_API_IMPORT __declspec(dllimport)
#  endif
#endif

#ifndef SHIORI_API_EXPORT
#  ifdef __cplusplus
#    define SHIORI_API_EXPORT extern "C" __declspec(dllexport)
#  else
#    define SHIORI_API_EXPORT __declspec(dllexport)
#  endif
#endif

#ifndef SHIORI_API
#  ifdef SHIORI_API_IMPLEMENTS
#    define SHIORI_API SHIORI_API_EXPORT
#  else
#    define SHIORI_API SHIORI_API_IMPORT
#  endif
#endif

/* ----------------------------------------------------------------------------
 * �x ���\�b�h
 */
SHIORI_API BOOL    __cdecl load(HGLOBAL    hGlobal_loaddir, long  loaddir_len);
SHIORI_API HGLOBAL __cdecl request(HGLOBAL hGlobal_request, long& len);
SHIORI_API BOOL    __cdecl unload(void);

#endif


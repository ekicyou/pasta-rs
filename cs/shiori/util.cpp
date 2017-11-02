// ----------------------------------------------------------------------------
// �ʃv���Z�X�ʐMSHIORI SHIOLINK2.DLL
//   The MIT License
//   http://sourceforge.jp/projects/opensource/wiki/licenses%2FMIT_license
// ----------------------------------------------------------------------------
#include "StdAfx.h"
#include "util.h"

/* ----------------------------------------------------------------------------
 * �G���[���b�Z�[�W�쐬
 */
void CreateBatRequestResponse(CharArray &res, LPCSTR msg)
{
	CAtlStringA text(      
		"SHIORI/3.0 500 Internal Server Error\r\n"
		"Charset: UTF-8\r\n"
		"Sender: SHIOLINK2\r\n"
		"X-SHIOLINK2-Reason: "
		);
	text += msg;
	text += "\r\n\r\n";
	res.SetCount(text.GetLength());
	::CopyMemory(res.GetData(), (LPCSTR)text, text.GetLength());
}
void CreateBatRequestResponse(CharArray &res, LPCTSTR msg)
{
	CreateBatRequestResponse(res, CT2CA(msg, 65001));
}

/* ----------------------------------------------------------------------------
 * Win32�G���[���b�Z�[�W�擾
 */
CString GetWinErrMessage(const HRESULT hr)
{
	TCHAR buf[_MAX_PATH];
	::FormatMessage(
		FORMAT_MESSAGE_FROM_SYSTEM, NULL, hr, 0, buf, sizeof(buf), NULL);
	CString mes;
	mes.Format(_T("[0x%X]%s"), hr, buf);
	return mes;
}

/**----------------------------------------------------------------------------
 * �J�����g�f�B���N�g���ړ������A
 */

Pushd::Pushd(LPCTSTR newdir)
	:mOldDir()
{
	TCHAR buf[_MAX_PATH+1];
	GetCurrentDirectory(sizeof(buf), buf);
	mOldDir = buf;
	BOOL rc = SetCurrentDirectory(newdir);
	if(!rc) AtlThrow( FAILED(ERROR_CURRENT_DIRECTORY) );
}

Pushd::~Pushd()
{
	if(mOldDir.IsEmpty()) return;
	SetCurrentDirectory(mOldDir);
}

/**----------------------------------------------------------------------------
 * �X�R�[�v���O�̏o��
 */

ScopeLog::ScopeLog(LPCTSTR fname)
	:mFuncName(fname)
{
    ATLTRACE2(_T("[%s]<<FUNC START>>\n"), mFuncName);
}

ScopeLog::~ScopeLog()
{
    ATLTRACE2(_T("[%s]<<FUNC END>>\n"), mFuncName);
}

// EOF
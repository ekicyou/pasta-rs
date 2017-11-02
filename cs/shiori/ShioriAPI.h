// ----------------------------------------------------------------------------
// �ʃv���Z�X�ʐMSHIORI SHIOLINK2.DLL
//   The MIT License
//   http://sourceforge.jp/projects/opensource/wiki/licenses%2FMIT_license
// ----------------------------------------------------------------------------
#pragma once
#include "util.h"

class CShioriAPI
{
private:
	HINSTANCE mInst;
	CPath     mLoadDir;

public:
	CShioriAPI(HINSTANCE hInst, LPCTSTR loadDir);
	virtual ~CShioriAPI(void);
public:
	// ���N�G�X�g���������A�������쐬���܂��B
	bool Request(LPCSTR req, const long reqLength, CharArray &res);
};

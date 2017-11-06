#include "stdafx.h"
#include "tuki.h"
using namespace System;

Setugekka::Tuki::Tuki() {
}

Setugekka::Tuki::~Tuki() {
}

BOOL Setugekka::Tuki::load(HINSTANCE hinst, HGLOBAL hGlobal_loaddir, long loaddir_len) {
    return FALSE;
}

BOOL  Setugekka::Tuki::unload(void) {
    return FALSE;
}

HGLOBAL Setugekka::Tuki::request(HGLOBAL hGlobal_request, long& len) {
    return nullptr;
}
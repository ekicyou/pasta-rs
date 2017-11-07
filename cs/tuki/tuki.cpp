#include "stdafx.h"
#include "tuki.h"
#include "assembly_utils.h"

using namespace System;
using namespace System::Text;
using namespace System::Runtime::InteropServices;

Setugekka::Tuki::Tuki() {
}

Setugekka::Tuki::~Tuki() {
}

void Setugekka::Tuki::CreateProxy(String^ load_dir) {
    auto bin_dir = System::IO::Path::Combine(load_dir, "bin");
    auto tmp_dir = System::IO::Path::Combine(load_dir, "temp");

    // 自分のディレクトリをパス解決に含める
    auto di(AssemblyUtil::AddCurrentAssemblyResolvePath(load_dir));

    // アプリケーションドメインの作成
    auto info = gcnew AppDomainSetup();
    info->ApplicationBase = load_dir;
    info->PrivateBinPath = bin_dir;
    info->DynamicBase = tmp_dir;
    info->ShadowCopyFiles = "true";
    auto name = gcnew StringBuilder();
    name->Append("TukiProxy_");
    name->Append(Guid::NewGuid().ToString());
    proxy_domain = AppDomain::CreateDomain(name->ToString(), nullptr, info);

    // プロキシを読み込む

    return;
}

BOOL Setugekka::Tuki::load(HINSTANCE hinst, HGLOBAL hGlobal_loaddir, long loaddir_len) {
    // プロキシの作成
    auto load_dir = Marshal::PtrToStringAnsi(IntPtr((void*)hGlobal_loaddir), loaddir_len);
    CreateProxy(load_dir);
    if (proxy == nullptr) return FALSE;
    return proxy->load(hinst, hGlobal_loaddir, loaddir_len);
}

BOOL Setugekka::Tuki::unload(void)
{
    auto rc = proxy != nullptr ? TRUE : FALSE;
    delete proxy;
    delete proxy_domain;
    proxy = nullptr;
    proxy_domain = nullptr;
    return rc;
}

HGLOBAL Setugekka::Tuki::request(HGLOBAL hGlobal_request, long & len)
{
    return HGLOBAL();
}
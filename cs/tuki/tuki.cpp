#include "stdafx.h"
#include "tuki.h"
#include "assembly_utils.h"

using namespace System;
using namespace System::Text;
using namespace System::Runtime::InteropServices;

Setugekka::Tuki::Tuki::Tuki() {
}

Setugekka::Tuki::Tuki::~Tuki() {
}

void Setugekka::Tuki::Tuki::CreateProxy(String^ load_dir) {
    auto bin_dir = System::IO::Path::Combine(load_dir, "bin");
    auto tmp_dir = System::IO::Path::Combine(load_dir, "temp");
    auto dyn_dir = System::IO::Path::Combine(tmp_dir, "dyn");
    auto sha_dir = System::IO::Path::Combine(tmp_dir, "shadow");

    // 自分のディレクトリをパス解決に含める
    auto di(AssemblyUtil::AddCurrentAssemblyResolvePath(load_dir));

    // アプリケーションドメインの作成
    auto name = "TukiProxy_" + Guid::NewGuid().ToString();
    auto info = gcnew AppDomainSetup();
    info->ApplicationName = name;
    info->ApplicationBase = load_dir;
    info->PrivateBinPath = bin_dir;
    info->DynamicBase = dyn_dir;
    info->ShadowCopyDirectories = sha_dir;
    info->ShadowCopyFiles = "true";
    proxy_domain = AppDomain::CreateDomain(name, nullptr, info);

    // プロキシを読み込む
    auto proxy_obj = proxy_domain->CreateInstanceAndUnwrap("yuki", "Setugekka.Yuki.Yuki");
    proxy = (Setugekka::IShioriUnsafe^)proxy_obj;
    return;
}

BOOL Setugekka::Tuki::Tuki::load(void* hinst, void* hGlobal_loaddir, long loaddir_len) {
    // プロキシの作成
    auto load_dir = Marshal::PtrToStringAnsi(IntPtr(hGlobal_loaddir), loaddir_len);
    CreateProxy(load_dir);
    if (proxy == nullptr) return FALSE;
    return proxy->load(hinst, hGlobal_loaddir, loaddir_len);
}

BOOL Setugekka::Tuki::Tuki::unload(void)
{
    auto rc = proxy != nullptr ? TRUE : FALSE;
    delete proxy;
    delete proxy_domain;
    proxy = nullptr;
    proxy_domain = nullptr;
    return rc;
}

HGLOBAL Setugekka::Tuki::Tuki::request(void* hGlobal_request, long & len)
{
    return HGLOBAL();
}
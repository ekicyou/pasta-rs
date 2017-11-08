#include "stdafx.h"
#include "tuki.h"
#include "assembly_utils.h"

using namespace System;
using namespace System::IO;
using namespace System::Text;
using namespace System::Runtime::InteropServices;

Setugekka::Tuki::Tuki::Tuki() {
}

Setugekka::Tuki::Tuki::~Tuki() {
}

void Setugekka::Tuki::Tuki::CreateProxy(String^ load_dir) {
    auto base_dir = Path::GetDirectoryName(
        Path::GetDirectoryName(load_dir));
    auto bin_dir = Path::Combine(load_dir, "bin");
    auto tmp_dir = Path::Combine(load_dir, "temp");
    auto dyn_dir = Path::Combine(tmp_dir, "dyn");
    auto sha_dir = Path::Combine(tmp_dir, "shadow");
    auto bin_dirs = load_dir + ";" + bin_dir;

    // 自分のディレクトリをパス解決に含める
    auto di(AssemblyUtil::AddCurrentAssemblyResolvePath(load_dir));

    // アプリケーションドメインの作成
    auto name = "TukiProxy_" + Guid::NewGuid().ToString();
    auto info = gcnew AppDomainSetup();
    info->ApplicationName = name;
    info->ApplicationBase = base_dir;
    info->PrivateBinPath = bin_dirs;
    info->PrivateBinPathProbe = bin_dirs;
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
    try {
        // プロキシの作成
        auto load_dir = Path::GetFullPath(
            Marshal::PtrToStringAnsi(IntPtr(hGlobal_loaddir), loaddir_len));
        CreateProxy(load_dir);
        if (proxy == nullptr) return FALSE;
        return proxy->load(hinst, hGlobal_loaddir, loaddir_len);
    }
    catch (...) {
        return FALSE;
    }
}

BOOL Setugekka::Tuki::Tuki::unload(void)
{
    try {
        auto rc = proxy != nullptr ? proxy->unload() : FALSE;
        delete proxy;
        if (proxy_domain != nullptr) AppDomain::Unload(proxy_domain);
        delete proxy_domain;
        proxy = nullptr;
        proxy_domain = nullptr;
        return rc;
    }
    catch (...) {
        return FALSE;
    }
}

HGLOBAL Setugekka::Tuki::Tuki::request(void* hGlobal_request, long & len)
{
    try {
        if (proxy != nullptr) return proxy->request(hGlobal_request, len);
        len = 0;
        return nullptr;
    }
    catch (...) {
        len = 0;
        return nullptr;
    }
}
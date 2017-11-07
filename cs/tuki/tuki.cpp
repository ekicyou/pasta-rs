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

    // �����̃f�B���N�g�����p�X�����Ɋ܂߂�
    auto di(AssemblyUtil::AddCurrentAssemblyResolvePath(load_dir));

    // �A�v���P�[�V�����h���C���̍쐬
    auto name = "TukiProxy_" + Guid::NewGuid().ToString();
    auto info = gcnew AppDomainSetup();
    info->ApplicationName = name;
    info->ApplicationBase = load_dir;
    info->PrivateBinPath = bin_dir;
    info->DynamicBase = dyn_dir;
    info->ShadowCopyDirectories = sha_dir;
    info->ShadowCopyFiles = "true";
    proxy_domain = AppDomain::CreateDomain(name, nullptr, info);

    // �v���L�V��ǂݍ���
    auto proxy_obj = proxy_domain->CreateInstanceAndUnwrap("yuki", "Setugekka.Yuki.Yuki");
    proxy = (Setugekka::IShioriUnsafe^)proxy_obj;
    return;
}

BOOL Setugekka::Tuki::Tuki::load(void* hinst, void* hGlobal_loaddir, long loaddir_len) {
    // �v���L�V�̍쐬
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
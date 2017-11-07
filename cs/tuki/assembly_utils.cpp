#include "stdafx.h"
#include "assembly_utils.h"

Assembly^ Setugekka::Tuki::AddCurrentAssemblyResolvePath_Deligate::Find(String^ path, String^ name)
{
    try
    {
        auto assembly = Assembly::LoadFile(path);
        return assembly;
    }
    catch (Exception^)
    {
        return nullptr;
    }
}

Assembly^ Setugekka::Tuki::AddCurrentAssemblyResolvePath_Deligate::CallBack(Object^ sender, ResolveEventArgs^ args)
{
    Debug::WriteLine(String::Format("AssemblyResolve: [{0}]", args->Name));

    auto m = re->Match(args->Name);
    if (!m->Success) return nullptr;

    auto assemblyName = m->Groups[1]->Value;
    auto version = m->Groups[2]->Value;
    auto culture = m->Groups[3]->Value;
    auto publicKeyToken = m->Groups[4]->Value;

    Assembly^ rc;
    String^ path;

    path = String::Format("{0}\\{1}.dll", dir, assemblyName);
    rc = Find(path, args->Name);
    return rc;
}

Setugekka::Tuki::AddCurrentAssemblyResolvePath_Deligate::AddCurrentAssemblyResolvePath_Deligate(AppDomain^ domain, String^ path)
{
    dir = Path::GetFullPath(path);
    re = gcnew Regex(
        "^([^,]*), Version=([^,]*), Culture=([^,]*), PublicKeyToken=(.*)$",
        RegexOptions::Compiled);
    this->domain = domain;
    h = gcnew ResolveEventHandler(this, &AddCurrentAssemblyResolvePath_Deligate::CallBack);
    AddEvent();
}
Setugekka::Tuki::AddCurrentAssemblyResolvePath_Deligate::~AddCurrentAssemblyResolvePath_Deligate() {
    RemoveEvent();
}
void Setugekka::Tuki::AddCurrentAssemblyResolvePath_Deligate::AddEvent() {
    domain->AssemblyResolve += h;
}
void Setugekka::Tuki::AddCurrentAssemblyResolvePath_Deligate::RemoveEvent() {
    domain->AssemblyResolve -= h;
}

String^ Setugekka::Tuki::AssemblyUtil::GetMethodName(int skipFrames)
{
    auto sf = gcnew StackFrame(skipFrames + 1);
    auto m = sf->GetMethod();
    return m->ReflectedType->Name + "::" + m->Name;
}

String^ Setugekka::Tuki::AssemblyUtil::GetMethodName()
{
    return GetMethodName(1);
}

String^ Setugekka::Tuki::AssemblyUtil::GetAssemblyPath(Assembly^ assembly)
{
    auto binURI = gcnew Uri(assembly->CodeBase);
    auto encPath = binURI->AbsolutePath;
    auto path = Uri::UnescapeDataString(encPath);
    auto fullPath = Path::GetFullPath(path);
    return fullPath;
}

String^ Setugekka::Tuki::AssemblyUtil::GetCallingAssemblyPath()
{
    return GetAssemblyPath(Assembly::GetCallingAssembly());
}

String^ Setugekka::Tuki::AssemblyUtil::GetAssemblyDirctory(Assembly^ assembly)
{
    return Path::GetDirectoryName(GetAssemblyPath(assembly));
}

String^ Setugekka::Tuki::AssemblyUtil::GetCallingAssemblyDirctory()
{
    return GetAssemblyDirctory(Assembly::GetCallingAssembly());
}

IDisposable^ Setugekka::Tuki::AssemblyUtil::AddCurrentAssemblyResolvePath(String^ path)
{
    auto cb = gcnew AddCurrentAssemblyResolvePath_Deligate(AppDomain::CurrentDomain, path);
    return cb;
}
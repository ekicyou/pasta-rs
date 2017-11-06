#include "stdafx.h"
#include "assembly_utils.h"

namespace Setugekka {
    Assembly^ AddCurrentAssemblyResolvePath_Deligate::Find(String^ path, String^ name)
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

    Assembly^ AddCurrentAssemblyResolvePath_Deligate::CallBack(Object^ sender, ResolveEventArgs^ args)
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

    AddCurrentAssemblyResolvePath_Deligate::AddCurrentAssemblyResolvePath_Deligate(AppDomain^ domain, String^ path)
    {
        dir = Path::GetFullPath(path);
        re = gcnew Regex(
            "^([^,]*), Version=([^,]*), Culture=([^,]*), PublicKeyToken=(.*)$",
            RegexOptions::Compiled);
        this->domain = domain;
        h = gcnew ResolveEventHandler(this, &AddCurrentAssemblyResolvePath_Deligate::CallBack);
        AddEvent();
    }
    AddCurrentAssemblyResolvePath_Deligate::~AddCurrentAssemblyResolvePath_Deligate() {
        RemoveEvent();
    }
    void AddCurrentAssemblyResolvePath_Deligate::AddEvent() {
        domain->AssemblyResolve += h;
    }
    void AddCurrentAssemblyResolvePath_Deligate::RemoveEvent() {
        domain->AssemblyResolve -= h;
    }

    String^ AssemblyUtil::GetMethodName(int skipFrames)
    {
        auto sf = gcnew StackFrame(skipFrames + 1);
        auto m = sf->GetMethod();
        return m->ReflectedType->Name + "::" + m->Name;
    }

    String^ AssemblyUtil::GetMethodName()
    {
        return GetMethodName(1);
    }

    String^ AssemblyUtil::GetAssemblyPath(Assembly^ assembly)
    {
        auto binURI = gcnew Uri(assembly->CodeBase);
        auto encPath = binURI->AbsolutePath;
        auto path = Uri::UnescapeDataString(encPath);
        auto fullPath = Path::GetFullPath(path);
        return fullPath;
    }

    String^ AssemblyUtil::GetCallingAssemblyPath()
    {
        return GetAssemblyPath(Assembly::GetCallingAssembly());
    }

    String^ AssemblyUtil::GetAssemblyDirctory(Assembly^ assembly)
    {
        return Path::GetDirectoryName(GetAssemblyPath(assembly));
    }

    String^ AssemblyUtil::GetCallingAssemblyDirctory()
    {
        return GetAssemblyDirctory(Assembly::GetCallingAssembly());
    }

    IDisposable^ AssemblyUtil::AddCurrentAssemblyResolvePath(String^ path)
    {
        auto cb = gcnew AddCurrentAssemblyResolvePath_Deligate(AppDomain::CurrentDomain, path);
        return cb;
    }
}
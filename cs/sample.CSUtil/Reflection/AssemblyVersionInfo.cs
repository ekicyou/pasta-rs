using System;
using System.Linq;
using System.Reflection;
using System.Text.RegularExpressions;

namespace CSUtil.Reflection
{
    /// <summary>
    /// アセンブリバージョン情報を取得します。
    /// </summary>
    public class AssemblyVersionInfo
    {
        /// <summary>対象のアセンブリ</summary>
        public Assembly Assembly { get; private set; }

        private T GetAttr<T>()
            where T : Attribute
        {
            var items = (T[])Assembly.GetCustomAttributes(typeof(T), false);
            return items.FirstOrDefault();
        }

        public AssemblyVersionInfo(Assembly assm)
        {
            Assembly = assm;

            L.Title = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyTitleAttribute>();
                if (attr == null) return "";
                return attr.Title;
            });
            L.Description = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyDescriptionAttribute>();
                if (attr == null) return "";
                return attr.Description;
            });
            L.Product = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyProductAttribute>();
                if (attr == null) return "";
                return attr.Product;
            });
            L.Company = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyCompanyAttribute>();
                if (attr == null) return "";
                return attr.Company;
            });
            L.Copyright = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyCopyrightAttribute>();
                if (attr == null) return "";
                return attr.Copyright;
            });
            L.Trademark = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyTrademarkAttribute>();
                if (attr == null) return "";
                return attr.Trademark;
            });
            L.Culture = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyCultureAttribute>();
                if (attr == null) return "";
                return attr.Culture;
            });
            L.Version = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyVersionAttribute>();
                if (attr == null) return "";
                return attr.Version;
            });
            L.FileVersion = new Lazy<string>(() =>
            {
                var attr = GetAttr<AssemblyFileVersionAttribute>();
                if (attr == null) return "";
                return attr.Version;
            });

            var reSPACES = new Regex(@"\s+");

            L.ProductTitle = new Lazy<string>(() =>
            {
                var title = Product + " " + Description;
                var rc = reSPACES.Replace(title, " ");
                return rc;
            });
        }

        private struct LazyStore
        {
            public Lazy<string> Title;
            public Lazy<string> Description;
            public Lazy<string> Product;
            public Lazy<string> Company;
            public Lazy<string> Copyright;
            public Lazy<string> Trademark;
            public Lazy<string> Culture;
            public Lazy<string> Version;
            public Lazy<string> FileVersion;
            public Lazy<string> ProductTitle;
        }

        private LazyStore L = new LazyStore();

        public string Title { get { return L.Title.Value; } }

        public string Description { get { return L.Description.Value; } }

        public string Product { get { return L.Product.Value; } }

        public string Company { get { return L.Company.Value; } }

        public string Copyright { get { return L.Copyright.Value; } }

        public string Trademark { get { return L.Trademark.Value; } }

        public string Culture { get { return L.Culture.Value; } }

        public string Version { get { return L.Version.Value; } }

        public string FileVersion { get { return L.FileVersion.Value; } }

        public string ProductTitle { get { return L.ProductTitle.Value; } }
    }
}
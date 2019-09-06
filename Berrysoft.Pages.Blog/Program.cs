using System;
using System.IO;
using System.Text.Json;
using Berrysoft.Pages.Data;

namespace Berrysoft.Pages.Blog
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length == 2)
            {
                var title = args[0];
                var filename = args[1];
                var file = new FileInfo(filename);
                var (fn, fnex) = GetFilename(file);
                var post = new BlogPost
                {
                    Title = title,
                    Date = DateTime.Now,
                    Filename = fn,
                    Type = GetPostType(file)
                };
                string json = JsonSerializer.Serialize(post, new JsonSerializerOptions()
                {
                    WriteIndented = true
                });
                if (file.Exists)
                {
                    file.MoveTo(Path.Combine(file.Directory.FullName, fnex));
                }
                Console.WriteLine(json);
            }
        }

        static (string, string) GetFilename(FileInfo file)
        {
            string[] slices = file.Name.Split('.');
            string name = string.Join('.', slices[..^1]);
            var now = DateTime.Now;
            var filename = $"{name}_{now.Year}_{now.Month}_{now.Day}";
            var filenameWithEx = $"{name}_{now.Year}_{now.Month}_{now.Day}.{slices[^1]}";
            switch (slices[^1])
            {
                case "txt":
                case "htm":
                case "md":
                    return (filename, filenameWithEx);
                default:
                    return (filenameWithEx, filenameWithEx);
            }
        }

        static BlogPostType GetPostType(FileInfo file)
        {
            string[] slices = file.Name.Split('.');
            return slices[^1] switch
            {
                "htm" => BlogPostType.Html,
                "md" => BlogPostType.Markdown,
                _ => BlogPostType.Text
            };
        }
    }
}

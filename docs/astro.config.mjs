import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://dublette.espadat.com",
  integrations: [
    starlight({
      title: "dublette",
      description: "Deduplicate images, videos, and audio using perceptual hashing and acoustic fingerprints.",
      customCss: ["@espadat/docs-theme/styles/theme.css"],
      components: {
        Footer: "@espadat/docs-theme/components/footer.astro",
        ThemeProvider: "@espadat/docs-theme/components/theme-provider.astro",
        ThemeSelect: "@espadat/docs-theme/components/theme-select.astro",
      },
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/espadat-studio/dublette" },
      ],
      sidebar: [
        { label: "Home", link: "/" },
        {
          label: "Getting Started",
          items: [{ slug: "getting-started/installation" }, { slug: "getting-started/quick-start" }],
        },
        { label: "CLI Reference", slug: "cli-reference" },
        {
          label: "How It Works",
          items: [
            { slug: "how-it-works/perceptual-hashing" },
            { slug: "how-it-works/acoustic-fingerprinting" },
            { slug: "how-it-works/deduplication-process" },
          ],
        },
        { label: "Examples", slug: "examples" },
        { label: "About", items: [{ slug: "about/license" }] },
      ],
    }),
  ],
});

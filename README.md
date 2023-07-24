# Newsie

![Made with Rust](https://img.shields.io/badge/Made%20with%20Rust-f74c00?style=for-the-badge&logo=rust)
![Maintenance](https://img.shields.io/maintenance/yes/2023?style=for-the-badge)
![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/m/samyosm/newsie?style=for-the-badge)
![LICENSE](https://img.shields.io/github/license/samyosm/newsie?style=for-the-badge)
![Website](https://img.shields.io/website?url=http%3A%2F%2Fnewsie.samyos.me%2F&style=for-the-badge&label=API)

Newsie is a News API that allows you to locate articles and their content in
text format based on RSS feeds. In the future, I'm also hoping of introducing AI
tools to gauge relevancy to current events and sentiment analysis.

## Public API

### Supported

Categories:

- General
- Tech (soon)
- Health (soon)
- Culture (soon)
- Sports (soon)

Languages:

- French
- English

**Base Endpoint**: http://newsie.samyos.me/api/v1

### [/news/{date}](http://newsie.samyos.me/api/v1/news/)

The date must be in the following format: YYYY-MM-DD

Example: `http://newsie.samyos.me/api/v1/news/2023-07-23`

```json
[
  {
    "guid": "https://apnews.com/article/missing-rodeo-goat-south-texas-search-fbadb83ca067d344c11866782da89c05",
    "title": "Avid search for missing Texas rodeo goat bringing residents of a small rural county together",
    "desc": "The search for a rodeo goat that has been missing for more than a week has the residents of a rural South Texas county enthralled as they are using horses, ATVs and even contemplating utilizing a helicopter to find the missing animal.",
    "content": "<article><div id=\"readability-page-1\">\n                <main data-module=\"\" data-padding=\"none\">\n                    \n                    \n                        \n                            \n    <div><bsp-carousel data-module=\"\" data-slidesratio=\"3x2\">\n    \n\n    <div>\n        \n            \n                <div>\n        <bsp-carousel-read-more data-expand=\"ReadMore-expand\" data-main-class=\"ReadMore\" data-more-id=\"data-more-button-0\" data-less-id=\"data-less-button-0\">\n            <p><span>1 of 2<span> | </span></span><span><p>This photo provided by the Willacy County Livestock Show and Fair shows a rodeo goat named Willy, who went missing on July 15, 2023, in a rural South Texas county. The search for Willy has residents enthralled as they’re using horses, ATVs and even contemplating using a helicopter to locate the missing animal. (Alma Barron/Willacy County Livestock Show and Fair via AP)</p></span><span><p>ASSOCIATED PRESS</p></span></p>\n            \n        </bsp-carousel-read-more>\n    </div>\n            \n                <div>\n        <bsp-carousel-read-more data-expand=\"ReadMore-expand\" data-main-class=\"ReadMore\" data-more-id=\"data-more-button-1\" data-less-id=\"data-less-button-1\">\n            <p><span>2 of 2<span> | </span></span><span><p>This photo provided by the Willacy County Livestock Show and Fair shows a rodeo goat named Willy, who went missing on July 15, 2023, in a rural South Texas county. The search for Willy has residents enthralled as they’re using horses, ATVs and even contemplating using a helicopter to locate the missing animal. (Alma Barron/Willacy County Livestock Show and Fair via AP)</p></span><span><p>ASSOCIATED PRESS</p></span></p>\n            \n        </bsp-carousel-read-more>\n    </div>\n            \n        \n    </div>\n\n    <template data-bsp-carousel-overlay-template=\"\">\n        \n        <bsp-carousel>\n            <div>\n                \n                    \n                        <div>\n                                <div>\n        <bsp-carousel-read-more data-expand=\"ReadMore-expand\" data-main-class=\"ReadMore\" data-more-id=\"data-more-button-0\" data-less-id=\"data-less-button-0\">\n            <p><span>1 of 2<span> | </span></span><span><p>This photo provided by the Willacy County Livestock Show and Fair shows a rodeo goat named Willy, who went missing on July 15, 2023, in a rural South Texas county. The search for Willy has residents enthralled as they’re using horses, ATVs and even contemplating using a helicopter to locate the missing animal. (Alma Barron/Willacy County Livestock Show and Fair via AP)</p></span><span><p>ASSOCIATED PRESS</p></span></p>\n            \n        </bsp-carousel-read-more>\n    </div>\n                                <div>\n                                    <bsp-carousel-read-more data-expand=\"ReadMore-expand\" data-main-class=\"ReadMore\" data-more-id=\"data-more-button-0\" data-less-id=\"data-less-button-0\">\n                                        <p>1 of 2</p><p>This photo provided by the Willacy County Livestock Show and Fair shows a rodeo goat named Willy, who went missing on July 15, 2023, in a rural South Texas county. The search for Willy has residents enthralled as they’re using horses, ATVs and even contemplating using a helicopter to locate the missing animal. (Alma Barron/Willacy County Livestock Show and Fair via AP)</p><p>ASSOCIATED PRESS</p>\n                                    </bsp-carousel-read-more>\n                                </div>\n                            </div>\n                    \n                        <div>\n                                <div>\n        <bsp-carousel-read-more data-expand=\"ReadMore-expand\" data-main-class=\"ReadMore\" data-more-id=\"data-more-button-1\" data-less-id=\"data-less-button-1\">\n            <p><span>2 of 2<span> | </span></span><span><p>This photo provided by the Willacy County Livestock Show and Fair shows a rodeo goat named Willy, who went missing on July 15, 2023, in a rural South Texas county. The search for Willy has residents enthralled as they’re using horses, ATVs and even contemplating using a helicopter to locate the missing animal. (Alma Barron/Willacy County Livestock Show and Fair via AP)</p></span><span><p>ASSOCIATED PRESS</p></span></p>\n            \n        </bsp-carousel-read-more>\n    </div>\n                                <div>\n                                    <bsp-carousel-read-more data-expand=\"ReadMore-expand\" data-main-class=\"ReadMore\" data-more-id=\"data-more-button-1\" data-less-id=\"data-less-button-1\">\n                                        <p>2 of 2</p><p>This photo provided by the Willacy County Livestock Show and Fair shows a rodeo goat named Willy, who went missing on July 15, 2023, in a rural South Texas county. The search for Willy has residents enthralled as they’re using horses, ATVs and even contemplating using a helicopter to locate the missing animal. (Alma Barron/Willacy County Livestock Show and Fair via AP)</p><p>ASSOCIATED PRESS</p>\n                                    </bsp-carousel-read-more>\n                                </div>\n                            </div>\n                    \n                \n            </div>\n            \n            \n        </bsp-carousel>  \n    </template>\n</bsp-carousel>\n</div>\n\n\n                        \n                    \n\n                    <div>\n                                        <p>RAYMONDVILLE, Texas (AP) — First there was Gone Girl. Now there is Gone Goat.</p><p>The search for a rodeo goat that has been missing for more than a week has the residents of a rural South Texas county enthralled as they are using horses, ATVs and even contemplating utilizing a helicopter to find the missing animal.</p><p>Local businesses have donated nearly 90 prizes and gifts worth more than $5,000, including brisket, frescoes and salon service, as a reward for the person who finds the goat.</p>\n\n<div data-hide-authors=\"true\" data-hide-eyebrows=\"true\" data-hide-dates=\"true\" data-hide-descriptions=\"true\" data-image-position=\"right\" data-align-center=\"\" data-gtm-modulestyle=\"Hub Peeks List\" data-inverse-colors=\"\" data-module=\"\">\n\n    \n        \n            \n    <div><a aria-label=\"Police find remains of St. Paul school&#x2019;s &#x2018;beloved&#x2019; goat after animal went missing\" href=\"https://apnews.com/article/missing-goat-remains-found-school-st-paul-facdccf8c72032ccb7c73b8914d5bf04\" target=\"_blank\"><picture data-crop=\"medium-16x9\">\n    \n        <source type=\"image/webp\" width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/1723187/2147483647/strip/true/crop/1000x562+0+142/resize/500x281!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2Fe3%2F2f%2Fabcd0dd0e17b8c933f8819357336%2F81673baa6d704cc49948c57d4a5bffe0\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/a800cbd/2147483647/strip/true/crop/1000x562+0+142/resize/500x281!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2Fe3%2F2f%2Fabcd0dd0e17b8c933f8819357336%2F81673baa6d704cc49948c57d4a5bffe0\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    <img alt=\"This photo provided by Lynn Overvoorde shows a missing goat Hazelnut at Montessori school in St. Paul, Minn. Officers who responded to reports of &#x201C;suspicious items&#x201D; at an address about 2.5 miles (4.02 kilometers) from the school found the remains the goat in a plastic storage box, the Star Tribune reported, Friday, June 30, 2023. (Lynn Overvoorde via AP)\" width=\"500\" height=\"281\" src=\"https://dims.apnews.com/dims4/default/a800cbd/2147483647/strip/true/crop/1000x562+0+142/resize/500x281!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2Fe3%2F2f%2Fabcd0dd0e17b8c933f8819357336%2F81673baa6d704cc49948c57d4a5bffe0\" loading=\"lazy\"/>\n</picture>\n</a></div>\n\n\n        \n    \n\n    <div>\n        \n        \n        \n\n        \n    \n\n\n\n        \n    <p>\n        Police on Friday morning discovered the remains of a goat stolen earlier this week from a Montessori school in St. Paul.\n    </p>\n\n\n\n        \n   \n\n\n\n        \n\n\n        \n\n        \n    </div>\n\n    \n</div>\n\n\n    \n\n<p>“This has just gotten bigger than we ever dreamed. Our county is a really small county, about 20,000 population and a mostly agriculture, farming and ranching community. And we’re very much one big family ... So, we’re excited that everybody wants to find our goat,” said Alison Savage, president of the Willacy County Livestock Show and Fair. </p><p>Residents, including families, have been scouring cotton and sugar cane fields since the goat escaped from a pen in the county’s rodeo arena near Raymondville on July 15 following a youth rodeo. On Sunday, possible goat tracks were spotted in a cotton field near Lyford, south of Raymondville.</p>\n\n<p>When the goat first went missing, it didn’t have a name. But after a poll on the livestock show’s Facebook page, the goat was named Willy, short for Willacy County, Savage said. While the goat has a name, Savage said officials are not sure if Willy is a boy or a girl.</p><p>The livestock show has been posting regular updates on its <span><a href=\"https://www.facebook.com/willacy.fairauction\" target=\"_blank\" rel=\"noopener\">Facebook page</a></span>. The search has also been a boon for the livestock show, as residents and businesses have donated hundreds of dollars to make improvements to the nonprofit’s arena and other facilities.</p><p>“He’s hiding from us somewhere. But we’re getting closer. We’re going to find him” Savage said.</p>\n                                    </div>\n\n                    \n\n\n                    \n\n\n                    \n    \n\n\n\n                    \n    \n\n\n                </main>\n\n                \n                    \n                \n            </div></article>",
    "url": "https://apnews.com/article/missing-rodeo-goat-south-texas-search-fbadb83ca067d344c11866782da89c05",
    "source": "General news",
    "category": "General",
    "language": "English",
    "date": "2023-07-23"
  },
  {
    "guid": "https://apnews.com/article/magic-mushrooms-psilocybin-psychedelic-minneapolis-minnesota-c20c90441102f0da6859a762bc1efcf4",
    "title": "Minneapolis backs off arrests for psychedelic plant use",
    "desc": "Minneapolis is backing away from enforcing laws that criminalize buying psychedelic plants or using them in private.",
    "content": "<article><div id=\"readability-page-1\">\n                <main data-module=\"\" data-padding=\"none\">\n                    \n                    \n                        \n                            \n    <div><figure>\n    \n\n    \n        <picture data-crop=\"medium-3x2\">\n    \n        <source media=\"(min-width: 1280px)\" type=\"image/webp\" width=\"980\" height=\"653\" srcset=\"https://dims.apnews.com/dims4/default/393e6a2/2147483647/strip/true/crop/4107x2737+0+39/resize/980x653!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 1280px)\" width=\"980\" height=\"653\" srcset=\"https://dims.apnews.com/dims4/default/155dde8/2147483647/strip/true/crop/4107x2737+0+39/resize/980x653!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 1024px)\" type=\"image/webp\" width=\"820\" height=\"546\" srcset=\"https://dims.apnews.com/dims4/default/a0e6f0c/2147483647/strip/true/crop/4107x2735+0+40/resize/820x546!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 1024px)\" width=\"820\" height=\"546\" srcset=\"https://dims.apnews.com/dims4/default/5471f16/2147483647/strip/true/crop/4107x2735+0+40/resize/820x546!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 768px)\" type=\"image/webp\" width=\"1024\" height=\"683\" srcset=\"https://dims.apnews.com/dims4/default/bc4bfbc/2147483647/strip/true/crop/4107x2739+0+38/resize/1024x683!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 768px)\" width=\"1024\" height=\"683\" srcset=\"https://dims.apnews.com/dims4/default/f8dfbcb/2147483647/strip/true/crop/4107x2739+0+38/resize/1024x683!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 600px)\" type=\"image/webp\" width=\"767\" height=\"511\" srcset=\"https://dims.apnews.com/dims4/default/0481a13/2147483647/strip/true/crop/4107x2736+0+39/resize/767x511!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x,https://dims.apnews.com/dims4/default/db9f0b0/2147483647/strip/true/crop/4107x2736+0+39/resize/1534x1022!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(min-width: 600px)\" width=\"767\" height=\"511\" srcset=\"https://dims.apnews.com/dims4/default/5edb8ac/2147483647/strip/true/crop/4107x2736+0+39/resize/767x511!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x,https://dims.apnews.com/dims4/default/e250156/2147483647/strip/true/crop/4107x2736+0+39/resize/1534x1022!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(max-width: 599px)\" type=\"image/webp\" width=\"567\" height=\"378\" srcset=\"https://dims.apnews.com/dims4/default/4805564/2147483647/strip/true/crop/4107x2738+0+39/resize/567x378!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x,https://dims.apnews.com/dims4/default/903876f/2147483647/strip/true/crop/4107x2738+0+39/resize/1134x756!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source media=\"(max-width: 599px)\" width=\"567\" height=\"378\" srcset=\"https://dims.apnews.com/dims4/default/05ef6e6/2147483647/strip/true/crop/4107x2738+0+39/resize/567x378!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x,https://dims.apnews.com/dims4/default/d378059/2147483647/strip/true/crop/4107x2738+0+39/resize/1134x756!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source type=\"image/webp\" width=\"320\" height=\"213\" srcset=\"https://dims.apnews.com/dims4/default/35d169c/2147483647/strip/true/crop/4107x2734+0+41/resize/320x213!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x,https://dims.apnews.com/dims4/default/dc904bc/2147483647/strip/true/crop/4107x2734+0+41/resize/640x426!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source width=\"320\" height=\"213\" srcset=\"https://dims.apnews.com/dims4/default/90608b2/2147483647/strip/true/crop/4107x2734+0+41/resize/320x213!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x,https://dims.apnews.com/dims4/default/e2f5e52/2147483647/strip/true/crop/4107x2734+0+41/resize/640x426!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    <img alt=\"FILE - A vendor bags psilocybin mushrooms at a pop-up cannabis market on May 24, 2019. Minneapolis is backing away from enforcing laws that criminalize psychedelic plants. On Friday, July 21, 2023, Mayor Jacob Frey ordered police to stop using taxpayer dollars to enforce most laws against hallucinogenic plants, which include psilocybin mushrooms, ayahuasca and mescaline. (AP Photo/Richard Vogel, File)\" srcset=\"https://dims.apnews.com/dims4/default/90608b2/2147483647/strip/true/crop/4107x2734+0+41/resize/320x213!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 1x, https://dims.apnews.com/dims4/default/e2f5e52/2147483647/strip/true/crop/4107x2734+0+41/resize/640x426!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b 2x\" width=\"320\" height=\"213\" src=\"https://dims.apnews.com/dims4/default/90608b2/2147483647/strip/true/crop/4107x2734+0+41/resize/320x213!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F4f%2F4a%2F33c061b44c3595901b4a1f7e5737%2F02e11593248648ac8096a655fa23fd8b\" loading=\"lazy\"/>\n</picture>\n\n    \n\n    \n        <div><figcaption><p>FILE - A vendor bags psilocybin mushrooms at a pop-up cannabis market on May 24, 2019. Minneapolis is backing away from enforcing laws that criminalize psychedelic plants. On Friday, July 21, 2023, Mayor Jacob Frey ordered police to stop using taxpayer dollars to enforce most laws against hallucinogenic plants, which include psilocybin mushrooms, ayahuasca and mescaline. (AP Photo/Richard Vogel, File)</p> –</figcaption><p>ASSOCIATED PRESS</p></div>\n    \n</figure>\n</div>\n\n\n                        \n                    \n\n                    <div>\n                            \n\n                            \n\n                            <div><P>\n                <bsp-timestamp data-timestamp=\"1690156419000\" data-recent-thresholdinhours=\"1\">\n                    <template data-date-tpl=\"\">Published [hour]:[minute] [AMPM] [timezone], [monthFull] [day], [year]</template>\n                    \n                </bsp-timestamp>\n            </P>\n        \n    </div>\n\n                            \n                                \n                                    <div>\n                                        <p>MINNEAPOLIS (AP) — Minneapolis is backing away from enforcing laws that criminalize buying psychedelic plants or using them in private.</p><p>Mayor Jacob Frey on Friday ordered police to stop using taxpayer dollars to enforce most laws against hallucinogenic plants. Minneapolis still prioritizes enforcing laws against selling psychedelic plants, bringing them to schools or using them while driving.</p><p>Minneapolis Police Chief Brian O’Hara in a statement said he stands by the mayor’s decision. </p>\n\n<div data-align-center=\"\" data-module=\"\" data-gtm-modulestyle=\"Hub Peeks List\" data-inverse-colors=\"\" data-show-loadmore=\"true\" data-image-position=\"right\">\n        \n            \n                \n                    <div data-hide-authors=\"true\" data-hide-eyebrows=\"true\" data-hide-dates=\"true\" data-hide-descriptions=\"true\">\n\n    \n        \n            \n    <div><a aria-label=\"Here we go again: Broncos undergo yet another reset with Sean Payton trying to rescue Russell Wilson\" href=\"https://apnews.com/article/denver-broncos-russell-wilson-sean-payton-f7a1f0d766452a8131d15edba2479746\" target=\"_blank\"><picture data-crop=\"medium-16x9\">\n    \n        <source type=\"image/webp\" width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/a140721/2147483647/strip/true/crop/1290x725+0+457/resize/500x281!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F61%2Fed%2F42d2aafdcdbdec19ed5bec128a13%2Fc6bac7b56eb2425baf7583a77099cd3b\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/cfeb108/2147483647/strip/true/crop/1290x725+0+457/resize/500x281!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F61%2Fed%2F42d2aafdcdbdec19ed5bec128a13%2Fc6bac7b56eb2425baf7583a77099cd3b\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    <img alt=\"FILE - Denver Broncos head coach Sean Payton, front, chats with retired Broncos linebacker Karl Mecklenburg during NFL football practice, Wednesday, June 14, 2023, in Centennial, Colo. Sean Payton is back on the sideline and this time it's in Denver. The former Saints head coach was hired after spending a year in the broadcast booth. (AP Photo/David Zalubowski, File)\" width=\"500\" height=\"281\" src=\"https://dims.apnews.com/dims4/default/cfeb108/2147483647/strip/true/crop/1290x725+0+457/resize/500x281!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F61%2Fed%2F42d2aafdcdbdec19ed5bec128a13%2Fc6bac7b56eb2425baf7583a77099cd3b\" loading=\"lazy\"/>\n</picture>\n</a></div>\n\n\n        \n    \n\n    <div>\n        \n        \n        \n\n        \n    \n\n\n\n        \n    <p>\n        Sean Payton is back on the sideline and this time it’s in Denver. The former Saints head coach was hired after spending a year in the broadcast booth.\n    </p>\n\n\n\n        \n   \n\n\n\n        \n\n\n        \n\n        \n    </div>\n\n    \n</div>\n                \n            \n        \n            \n                \n                    <div data-hide-authors=\"true\" data-hide-eyebrows=\"true\" data-hide-dates=\"true\" data-hide-descriptions=\"true\">\n\n    \n        \n            \n    <div><a aria-label=\"Marlon Wayans cited after luggage dispute with United worker at Denver airport\" href=\"https://apnews.com/article/marlon-wayans-united-airlines-luggage-citation-ca8928fc21e069cf94058bfbd9291031\" target=\"_blank\"><picture data-crop=\"medium-16x9\">\n    \n        <source type=\"image/webp\" width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/492b1ff/2147483647/strip/true/crop/2500x1405+0+128/resize/500x281!/format/webp/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F18%2F02%2F05e6f9c9328d6955f98f196d30f3%2Ff3e0cb4540ce4974ba584b3f16d23a42\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/8107e0c/2147483647/strip/true/crop/2500x1405+0+128/resize/500x281!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F18%2F02%2F05e6f9c9328d6955f98f196d30f3%2Ff3e0cb4540ce4974ba584b3f16d23a42\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    <img alt=\"FILE - Marlon Wayans attends the &quot;Marlon&quot; FYC Event at UCB Theatre, June 6, 2018, in Los Angeles. Wayans was cited for disturbing the peace after a disturbance with a United Airlines employee at Denver's airport last week, police said Monday, June 12, 2023. (Photo by Richard Shotwell/Invision/AP, File)\" width=\"500\" height=\"281\" src=\"https://dims.apnews.com/dims4/default/8107e0c/2147483647/strip/true/crop/2500x1405+0+128/resize/500x281!/quality/90/?url=https%3A%2F%2Fassets.apnews.com%2F18%2F02%2F05e6f9c9328d6955f98f196d30f3%2Ff3e0cb4540ce4974ba584b3f16d23a42\" loading=\"lazy\"/>\n</picture>\n</a></div>\n\n\n        \n    \n\n    <div>\n        \n        \n        \n\n        \n    \n\n\n\n        \n    <p>\n        Comedian and actor Marlon Wayans was cited for disturbing the peace after a dispute he says he had last week with a United Airlines employee over carry-on luggage at the Denver airport.\n    </p>\n\n\n\n        \n   \n\n\n\n        \n\n\n        \n\n        \n    </div>\n\n    \n</div>\n                \n            \n        \n            \n                \n                    <div data-hide-authors=\"true\" data-hide-eyebrows=\"true\" data-hide-dates=\"true\" data-hide-descriptions=\"true\">\n\n    \n        \n            \n    <div><a aria-label=\"Black teen shot by officer during struggle was armed with pellet gun, not handgun, police say\" href=\"https://apnews.com/article/jordell-richardson-police-kill-black-teen-video-508a87bf79ed41cd35ab8e579cfcc72a\" target=\"_blank\"><picture data-crop=\"medium-16x9\">\n    \n        <source type=\"image/webp\" width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/c87257c/2147483647/strip/true/crop/3000x1686+0+282/resize/500x281!/format/webp/quality/90/?url=https%3A%2F%2Fstorage.googleapis.com%2Fafs-prod%2Fmedia%2Fd88fa800bddf4d3caee122da087d4669%2F3000.jpeg\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/12b7aa9/2147483647/strip/true/crop/3000x1686+0+282/resize/500x281!/quality/90/?url=https%3A%2F%2Fstorage.googleapis.com%2Fafs-prod%2Fmedia%2Fd88fa800bddf4d3caee122da087d4669%2F3000.jpeg\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    <img alt=\"Aurora Interim Chief of Police Art Acevedo speaks during a press conference, Friday, June 9, 2023 in Aurora, Colo. Aurora Police planned to release body camera footage showing an officer fatally shooting a 14-year-old Black boy they say was armed with a pellet gun. (Grace Smith/The Denver Post via AP)\" width=\"500\" height=\"281\" src=\"https://dims.apnews.com/dims4/default/12b7aa9/2147483647/strip/true/crop/3000x1686+0+282/resize/500x281!/quality/90/?url=https%3A%2F%2Fstorage.googleapis.com%2Fafs-prod%2Fmedia%2Fd88fa800bddf4d3caee122da087d4669%2F3000.jpeg\" loading=\"lazy\"/>\n</picture>\n</a></div>\n\n\n        \n    \n\n    <div>\n        \n        \n        \n\n        \n    \n\n\n\n        \n    <p>\n        Police in suburban Denver say a Black teen fatally shot by an officer was armed with a pellet gun and not a semiautomatic handgun as they originally had said.\n    </p>\n\n\n\n        \n   \n\n\n\n        \n\n\n        \n\n        \n    </div>\n\n    \n</div>\n                \n            \n        \n            \n                \n                    <div data-hide-authors=\"true\" data-hide-eyebrows=\"true\" data-hide-dates=\"true\" data-hide-descriptions=\"true\">\n\n    \n        \n            \n    <div><a aria-label=\"James Watt, sharp-tongued and pro-development Interior secretary under Reagan, dies at 85\" href=\"https://apnews.com/article/james-watt-interior-secretary-dead-c9119f1ed545653ae8ccf4f7e14bf327\" target=\"_blank\"><picture data-crop=\"medium-16x9\">\n    \n        <source type=\"image/webp\" width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/f1627e4/2147483647/strip/true/crop/2998x1685+0+230/resize/500x281!/format/webp/quality/90/?url=https%3A%2F%2Fstorage.googleapis.com%2Fafs-prod%2Fmedia%2F6dcc62c943024e96ae699aaba673bb0f%2F2998.jpeg\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    \n        <source width=\"500\" height=\"281\" srcset=\"https://dims.apnews.com/dims4/default/d77d8b9/2147483647/strip/true/crop/2998x1685+0+230/resize/500x281!/quality/90/?url=https%3A%2F%2Fstorage.googleapis.com%2Fafs-prod%2Fmedia%2F6dcc62c943024e96ae699aaba673bb0f%2F2998.jpeg\" loading=\"lazy\"><empty></empty></source>\n\n    \n\n    <img alt=\"FILE - Interior Secretary nominee James Watt, a Reagan nominee on talks on Dec. 23, 1980 in Washington. Reagan administration Interior Secretary James Watt has died at age 85. Watt died in Arizona on May 27, 2023, son Eric Watt said in a statement Thursday, June 8. (AP Photo/Taylor, File)\" width=\"500\" height=\"281\" src=\"https://dims.apnews.com/dims4/default/d77d8b9/2147483647/strip/true/crop/2998x1685+0+230/resize/500x281!/quality/90/?url=https%3A%2F%2Fstorage.googleapis.com%2Fafs-prod%2Fmedia%2F6dcc62c943024e96ae699aaba673bb0f%2F2998.jpeg\" loading=\"lazy\"/>\n</picture>\n</a></div>\n\n\n        \n    \n\n    <div>\n        \n        \n        \n\n        \n    \n\n\n\n        \n    <p>\n        Reagan administration Interior Secretary James Watt has died at age 85. The sharp-tongued Watt drew frequent criticism from environmentalists over what they saw as pro-development policies that included expanded logging and drilling on public lands.\n    </p>\n\n\n\n        \n   \n\n\n\n        \n\n\n        \n\n        \n    </div>\n\n    \n</div>\n                \n            \n        \n        </div>\n\n\n    \n\n<p>Announcing the order, Frey cited the potential for hallucinogenic plants to treat mental illnesses including depression and post-traumatic stress disorder.</p><p>“Experts are telling us that these plants help people, and that’s the business we should be in – helping people,” Frey said in a statement. “With a rise in deaths of despair in our city, and in our society, the data is showing that these plants can help be a remedy.”</p>\n\n<p>Some researchers believe psilocybin, the compound in psychedelic mushrooms, <span><a href=\"https://apnews.com/article/mushroom-psychedelic-alcoholism-study-a3b6692ae7590de9fd09a7cac271a199\" target=\"_blank\" rel=\"noopener\">changes the way</a></span> the brain organizes itself and can help users overcome things like depression, alcoholism and post-traumatic stress disorder. A drug that’s related <span><a href=\"https://apnews.com/article/6bf8d3dbe4c2411894635f11418b74dc\" target=\"_blank\" rel=\"noopener\">to the anesthetic ketamine</a></span> was cleared by the FDA to <span><a href=\"https://apnews.com/article/8f1a189bfde14cbf91179a60b900a81a\" target=\"_blank\" rel=\"noopener\">help people with hard-to-treat depression</a></span>.</p><p>But medical experts caution that more research is needed on the drugs’ efficacy and the extent of the risks of psychedelics, which can cause hallucinations.</p><p>The American Psychiatric Association <span><a href=\"https://www.psychiatry.org/getattachment/d5c13619-ca1f-491f-a7a8-b7141c800904/Position-Use-of-Psychedelic-Empathogenic-Agents.pdf\" target=\"_blank\" rel=\"noopener\">has not endorsed the use of psychedelics in treatment</a></span>, noting the Food and Drug Administration has yet to offer a final determination. The FDA designated psilocybin as a “breakthrough therapy” in 2018, a label that’s designed to speed the development and review of drugs to treat a serious condition. MDMA, also known as ecstasy, also has that designation for PTSD treatment.</p><p>The FDA in June released <span><a href=\"https://www.fda.gov/regulatory-information/search-fda-guidance-documents/psychedelic-drugs-considerations-clinical-investigations?utm_medium=email&amp;utm_source=govdelivery\" target=\"_blank\" rel=\"noopener\">draft guidance</a></span> for researchers designing clinical trials testing psychedelic drugs as potential treatments for a variety of medical conditions. The Biden administration has also provided to the National Institutes of Health and other agencies funding for <span><a href=\"https://psychedelicalpha.com/wp-content/uploads/2022/07/NIH-Psychedelics-Letter-June-2022.pdf\" target=\"_blank\" rel=\"noopener\">dozens of projects studying psychedelic drugs</a></span> with potential benefit for mental and behavioral health.</p>\n    \n\n<p>Earlier this year, Oregon became the first state in the nation to <span><a href=\"https://apnews.com/article/marijuana-oregon-elections-a7bf2c4477ca38a31bd140848c023061\" target=\"_blank\" rel=\"noopener\">legalize the adult use of psilocybin.</a></span> Colorado’s voters last year voted to decriminalize psilocybin.</p><p>Denver was the first city to decriminalize personal possession and consumption of psilocybin in 2019.</p><p>A Minneapolis-based organization that advocates for immigrant rights and criminal-justice reform touted the mayor’s order.</p><p>“This is an important first step to undo all the harms inflicted from the war on people who use drugs, which was created to target brown and Black peoples,” DecriMN Coalition founding member Jessica Nielson said in a statement. “These natural medicines and their use by Indigenous peoples predate any of these laws.”</p>\n                                    </div>\n                                \n                            \n\n                            \n                    </div>\n\n                    \n\n\n                    \n    \n\n\n\n                    \n    \n\n\n\n                    \n    \n\n\n                </main>\n\n                \n                    \n                \n            </div></article>",
    "url": "https://apnews.com/article/magic-mushrooms-psilocybin-psychedelic-minneapolis-minnesota-c20c90441102f0da6859a762bc1efcf4",
    "source": "General news",
    "category": "General",
    "language": "English",
    "date": "2023-07-23"
  }
]
```

### [/fetch?authorization={api_key}](http://newsie.samyos.me/api/v1/fetch?authorization=)

This is an admin-only command that restarts the pipeline to fetch newer articles.
It is called every 30mins by a cron job.

## Deploy your own API

1. Copy the `production.yaml` file and the `batches` directory.
2. Setup a `.env` file with your `FETCH_AUTHORIZATION` variable (to restart the
   pipeline)
3. Run `docker compose up -d`

You can customize the RSS Feeds of your API by changing the files in your
`batches` directory. You can add or remove files as you see fit. For an example
please refer to this repo's `batches` directory.

### Testing Purposes

1. Clone this repository
2. Run `docker compose up -d`

## Pipeline

1. RSS Feeds
2. Articles
3. Extract Article Text

## License

[GPL-3.0](./LICENSE)

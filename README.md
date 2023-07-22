# Newsie

Newsie is a news API that allows you to locate articles and their content in
text format based on RSS feeds. In the future, I'm also hoping of introducing AI
tools to gauge relevency to current events and sentiment anaylsis.

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

**Base Endpoint**: https://18.217.34.51/api/

### /v1/news/{date}

The date must be in the following format: YYYY-MM-DD

Example: `https://18.217.34.51/api/v1/news/2023-07-21`

```json
[
  {
    "guid": "963c6838-27e3-11ee-8de7-005056bf762b",
    "title": "Tour de France : le Slovène Matej Mohoric savoure une victoire pleine d'émotion",
    "desc": "Le Slovène Matej Mohoric (Bahrain) a remporté la 19e étape du Tour de France, vendredi, en devançant le Danois Kasper Asgreen à la photo-finish, à Poligny. ",
    "content": "PublicitéUne victoire slovène. Matej Mohoric (Bahrain) a remporté la 19e étape du Tour de France en devançant le Danois Kasper Asgreen à la photo-finish, vendredi 21 juillet, à Poligny.\nC'est la troisième victoire dans la Grande Boucle pour le Slovène, qui s'est imposé devant ses deux compagnons d'échappée, Asgreen et l'Australien Ben O'Connor, au bout d'une longue ligne droite finale de 8 km.\nLarmes de vainqueur\"C'est vraiment une victoire pleine d'émotion pour moi, je savais que c'était la plus grande occasion, et de loin, pour moi de gagner une étape. J'ai tout sacrifié pour arriver prêt sur ce Tour, je savais que j'avais les jambes pour gagner\", a commenté le Slovène victorieux.\nGrand spécialiste de la descente, Mohoric, 28 ans, a jeté son vélo sur la ligne pour garder quelques millimètres d'avance sur Asgreen, vainqueur la veille à Bourg-en-Bresse.\nC'était tellement serré qu'il a dû patienter quelques minutes après l'arrivée pour être assuré de sa victoire, avant d'éclater en sanglots. \"On fait de son mieux mais on souffre dans la roue des autres, tout le monde est tellement fort\", s'est expliqué le cycliste.\nLe compatriote de Tadej Pogacar avait déjà levé les bras à deux reprises sur le Tour en 2021 et compte également un Monument à son palmarès avec sa victoire à Milan-Sanremo en 2022.\nTadej Pogacar, 2e du classement général après la 19e étape du Tour de France arrivée à Poligny, s'est réjoui du résultat de l'étape : \"J'étais très content pour lui, content de voir un ami et un compatriote gagner cette étape, surtout avec ce style, cette élégance\".\nC'est aussi, après Pello Bilbao et Wout Poels, la troisième victoire dans ce Tour de France pour l'équipe Bahrain, endeuillée juste deux semaines avant le départ par la mort de son coureur suisse Gino Mäder dans une descente de col au Tour de Suisse .\nMohoric s'est montré le plus costaud et le plus malin d'une étape assez dingue, courue comme une classique, avec une multitude d'attaques, de tentatives d'échappée et des coureurs éclatés dans plusieurs groupes.\nLe peloton, avec le maillot jaune Jonas Vingegaard , est arrivé avec près de 14 minutes de retard, roulant sur un rythme de sénateur puisqu'aucun coureur dangereux pour le classement général ne s'était glissé dans l'échappée.\nAvec AFPLe résumé de la semaineFrance 24 vous propose de revenir sur les actualités qui ont marqué la semaine\nEmportez l'actualité internationale partout avec vous ! Téléchargez l'application France 24\n",
    "url": "https://www.france24.com/fr/sports/20230721-tour-de-france-le-slov%C3%A8ne-matej-mohoric-savoure-une-victoire-pleine-d-%C3%A9motion",
    "source": "France 24 - Infos, news & actualités - L'information internationale en direct",
    "category": "General",
    "language": "French",
    "date": "2023-07-21"
  },
  {
    "guid": "29ccb922-27c1-11ee-9b04-005056a97652",
    "title": "L'actu en dessin : au gré du dérèglement climatique, la planète suffoque et se noie",
    "desc": "Avec des températures flirtant les 50 °C, de Pékin à Rome, l'hémisphère Nord suffoque. Ou brûle, comme au Canada et en Grèce. Mais le dérèglement climatique, qui accélère la montée des eaux sur les côtes, provoque aussi des pluies diluviennes, comme cette semaine en Inde, en Corée, ou au Japon. Déréglée, la planète suffoque et ne noie tout à la fois. Un double défi, qui se raconte en dessin. ",
    "content": "Publicité53 °C aux États-Unis, 52 °C en Chine, 48 °C en Sardaigne… Après le mois de juin le plus chaud de l’Histoire, le mois de juillet est marqué par des vagues de chaleur exceptionnelles dans tout l’hémisphère Nord. Le record de la plus haute chaleur jamais enregistrée, 54,4 °C à l’ombre, pourrait même être battu dans la bien nommée \"Vallée de la mort\" aux États-Unis.\nCette météo suffocante fait craindre un peu partout des incendies similaires à ceux qui ont ravagé le Canada ou qui sévissent actuellement en Grèce , mais aussi des vagues de décès chez les plus vulnérables. Les dômes de chaleur responsables de ce phénomène global ont un lien indéniable avec le réchauffement climatique.\nLe Japon a émis, quant à lui, des alertes aux coups de chaleur lundi pour 32 de ses 47 préfectures, qui connaissent des températures proches du record absolu de 41,1 °C atteint en 2018.\nMultiplicité des périls climatiquesCe pays fait également face à des pluies torrentielles qui ont fait près d'une dizaine de morts. Ces intempéries menacent plusieurs régions d'Asie, comme le nord de l'Inde où des dizaines de milliers de foyers ont vu leurs habitations inondées. La Corée du sud est particulièrement endeuillée, avec des dizaines de morts également.\nEt comme la chaleur, la menace des eaux va s'aggraver. En Méditerranée, le niveau de la mer a augmenté de 2,8 mm par an au cours des dernières décennies, menaçant les rivages et les villes comme Venise, qui subit de plus en plus d'inondations.\n\"L'élévation du niveau de la mer touche déjà les eaux côtières du pourtour méditerranéen et devrait augmenter les risques d'inondation, d'érosion et de salinisation des côtes\", souligne le Giec.\nEn 2023, la chaleur, les sécheresses, et les mégafeux le disputent aux inondations. C'est cette multiplicité de périls climatiques qu'a illustrée le dessinateur Pedro x. Molina .\nDepuis plus de vingt ans, cet artiste originaire du Nicaragua  raconte le monde en dessins partout sur la planète.\nCes œuvres ont été publiées par des journaux de renommée mondiale comme Politico, Los Angeles Times, The Washington Post, Courrier International. Aujourd’hui, ses dessins paraissent quotidiennement sur Confidencial.com.ni . Il a également contribué à la collection Cartooning for Peace aux éditions Gallimard.\nest un réseau international de dessinateurs engagés à promouvoir, par l'universalité du dessin de presse, la liberté d'expression, les droits humains et le respect mutuel entre des populations de différentes cultures ou croyances.\nLe résumé de la semaineFrance 24 vous propose de revenir sur les actualités qui ont marqué la semaine\nEmportez l'actualité internationale partout avec vous ! Téléchargez l'application France 24\n",
    "url": "https://www.france24.com/fr/plan%C3%A8te/20230721-l-actu-en-dessin-au-gr%C3%A9-du-d%C3%A9r%C3%A8glement-climatique-la-plan%C3%A8te-suffoque-et-se-noie",
    "source": "France 24 - Infos, news & actualités - L'information internationale en direct",
    "category": "General",
    "language": "French",
    "date": "2023-07-21"
  },
  {
    "guid": "9a02430a-2704-11ee-8a78-005056bf762b",
    "title": "Enquête : Russie-Afrique, les réseaux d’influence",
    "desc": "En France, les influenceurs panafricains comptent de plus en plus d’adeptes sur les réseaux sociaux. Prônant une idéologie radicale anti-occidentale et souverainiste, certains d’entre eux revendiquent la paternité des changements de régime au Mali et au Burkina Faso. Un narratif proche de celui du Kremlin qui tente d’avancer ses pions sur le continent. France 24 dresse le portrait de deux influenceurs français qui semblent avoir pris fait et cause pour le président russe Vladimir Poutine.",
    "content": "Enquête : Russie-Afrique, les réseaux d’influencePar :En France, les influenceurs panafricains comptent de plus en plus d’adeptes sur les réseaux sociaux. Prônant une idéologie radicale anti-occidentale et souverainiste, certains d’entre eux revendiquent la paternité des changements de régime au Mali et au Burkina Faso. Un narratif proche de celui du Kremlin qui tente d’avancer ses pions sur le continent. France 24 dresse le portrait de deux influenceurs français qui semblent avoir pris fait et cause pour le président russe Vladimir Poutine.\nPublicitéCette enquête fait partie d'un travail d'investigation mené par France 24 et RFI, \"Influences russes en Afrique\", une série de reportages à retrouver sur rfi.fr et france24.com .Podcasts à écouter sur RFI\nLe résumé de la semaineFrance 24 vous propose de revenir sur les actualités qui ont marqué la semaine\nEmportez l'actualité internationale partout avec vous ! Téléchargez l'application France 24\nContenus liés",
    "url": "https://www.france24.com/fr/%C3%A9missions/reporters/20230721-enqu%C3%AAte-russie-afrique-les-r%C3%A9seaux-d-influence",
    "source": "France 24 - Infos, news & actualités - L'information internationale en direct",
    "category": "General",
    "language": "French",
    "date": "2023-07-21"
  }
]
```

### /v1/fetch?authorization={api_key}

This is an admin only command that restart the pipeline to fetch newer articles.
It is called every 30mins by a cron job.

## Deploy your own API

1. Copy the `production.yaml` file and the `batches` directory.
2. Setup a `.env` file with your `FETCH_AUTHORIZATION` variable (to restart the
   pipeline)
3. Run `docker compose up -d`

You can customize the RSS Feeds of your API by changing the files in your
`batches` directory. You can add or remove file as you see fit. For an example
please refer to this repo's `batches` directory.

### Testing Purposes

1. Clone this repository
2. Run `docker compose up -d`

## Pipeline

1. RSS Feeds
2. Articles
3. Extract Article Text

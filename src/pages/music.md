---
layout: ../layouts/PageLayout.astro
title: "Musik"
description: "Koleksi Playlist Musik Favorit"
---

Selamat datang di halaman musik! Dengarkan playlist pilihan NoCopyrightSounds, trending tracks Audius, dan lagu anime pilihan.

{% media audio %}
- title: NoCopyrightSounds (NCS)
  list:
    - https://audius.co/ncsounds
- title: Trending Audius
  list:
    - https://audius.co/trending?timeRange=week
- title: Trending Hip-Hop / Rap
  list:
    - https://audius.co/trending?genre=Hip-Hop%2FRap&timeRange=week
- title: Anime & Vocaloid
  list:
    - https://music.163.com/playlist?id=8676645748
    - https://music.163.com/playlist?id=8882890045
{% endmedia %}

HabitRPG viewer
===============

Fetch [HabitRPG][] information as a command line tool.

See <https://habitrpg.com/static/api#!/> for API.

[HabitRPG]: http://habitrpg.com/ "HabitRPG"

Setup
-----

Create `id.json` with

```
{
    "api_token": "your api token",
    "user_id": "your user id"
}
```

Needs
-----

1. Unmarked dailies
2. To-Dos
3. Sort by tags
4. Fetch todo due dates
5. Profile info (life, xp)
6. Boss info
7. Mark To-Dos
8. Mark dailies
9. Take rewards
10. Dailies done stats
11. Todos done today/this week stats

TODO
====

1. Impl for json Decodable, use Decoder to fetch info.


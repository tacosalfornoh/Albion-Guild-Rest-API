### Database:
- Surreal DB

### Todo List:
- discord_controller rimuovere .clone e usare i & riferimenti
- rimuovere .unwrap o .expect s sostituirli con Enum::Result

### API List:
- [PUT] /discords body{discord_id,discord_name,joined_at,...}
- [PATCH] /siscords/<discord_id>/name body{new_name}
- [PATCH] /discords/<discord_id>/balance body{amount}
- [GET] /discords/<discord_id>/balance
- [PUT] /users body{user_id,username,balance,...}
- [PATCH] /users/<user_id>/username body{new_username}
- [PATCH] /users/<user_id>/vod body{points}
- [PATCH] /users/<user_id>/link body{ign_name}
- [PATCH] /users/<user_id>/attendance body{points}
- [PATCH] /users/<user_id>/balance body{amount}
- [DELETE] /users/<user_id>/link
- [GET] /users/<user_id>/balance
- [PUT] /comps body{name,content,description}
- [PATCH] /comps/<comp_id>/name body{new_name}
- [PATCH] /comps/<comp_id>/description body{new_description}
- [DELETE] /comps/<comp_id>
- [PUT] /builds/ body{weapon,role,cape,...}
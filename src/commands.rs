use crate::{models::{Character, Stat, User}, schema::{stats, users::{self, active_char, id}}, Context, Data, Error};
use diesel::{BoolExpressionMethods, query_dsl::methods::FilterDsl, ExpressionMethods};
use poise::{self, serenity_prelude::{self as serenity, ComponentInteraction, Mentionable}};
use diesel_async::RunQueryDsl;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, poise::Modal)]
#[allow(dead_code)]
struct StatModal {
    name: String,
    value: String,
}

fn display_character(name: &str, owner: serenity::Mention, stats: &Vec<(String, i32)>) -> String {
    let formatted_stats = match stats.len() {
        0 => "*No stats. Click the button to add some.*".to_string(),
        _ => stats.iter()
            .map(|(name, value)| format!("{}: {}", name, value))
            .collect::<Vec<String>>()
            .join("\n")
    };
    
    return format!(
        "Name: {}\nOwner: {}\n\n{}\n\n*Stat values must be integers.*",
        name,
        owner,
        formatted_stats
    );
}

const VALID_BUTTON_IDS: [&str; 2] = ["add_stat", "finish_creation"];

#[poise::command(slash_command)]
pub async fn create_character(ctx: Context<'_>, name: String, ) -> Result<(), Error> {    
    let Data { pool } = ctx.data();

    let mut stats: Vec<(String, i32)> = Vec::new();

    let base_reply = poise::CreateReply::default()
        .components(vec![
            serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("add_stat")
                    .label("Add Stat"),
                serenity::CreateButton::new("finish_creation")
                    .label("Finish")
            ])
        ]);

    let reply = ctx.send(
        base_reply
            .clone()
            .content(display_character(&name, ctx.author().mention(), &stats))
    ).await?;

    while let Some(interaction) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .filter(move |interaction| VALID_BUTTON_IDS.contains(&interaction.data.custom_id.as_str()))
        .await
    {
        let ComponentInteraction { ref data, .. } = interaction;
        match data.custom_id.as_str() {
            "add_stat" => {
                let data = poise::modal::execute_modal_on_component_interaction::<StatModal>(ctx, interaction, None, None).await?;
                if let Some(data) = data {
                    if let Ok(value) = data.value.parse::<i32>() {
                        stats.push((data.name, value));
                        reply.edit(ctx, 
                            base_reply
                                .clone()
                                .content(display_character(&name, ctx.author().mention(), &stats))
                        ).await?;
                    }
                }
            },
            "finish_creation" => {
                Character::create_character_with_stats(&pool, Character { name, owner: ctx.author().id.get() as i64 }, stats).await?;            
                interaction.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                break;
            },
            _ => {}
        };
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn select_character(ctx: Context<'_>, name: String) -> Result<(), Error> {
    let Data { pool } = ctx.data();
    let mut conn = pool.get().await?;

    diesel::insert_into(users::table)
        .values(&User { active_char: name.clone(), id: ctx.author().id.get() as i64 })
        .on_conflict(id)
        .do_update()
        .set(active_char.eq(name))
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn roll(ctx: Context<'_>, stat_name: String) -> Result<(), Error> {
    let Data { pool } = ctx.data();
    let mut conn = pool.get().await?;
   
    let author_id = ctx.author().id.get() as i64;

    let user: Result<User, _> = users::table
        .filter(id.eq(author_id))
        .first(&mut conn)
        .await;

    if let Ok(user) = user {
        let stat: Stat = stats::table
            .filter(stats::char_owner.eq(author_id)
                .and(stats::char_name.eq(user.active_char))
                .and(stats::name.eq(stat_name))
            )
            .first(&mut conn)
            .await?;

        let mut rng = StdRng::from_entropy();
        let die1 = rng.gen_range(1..=6);
        let die2 = rng.gen_range(1..=6);
        ctx.reply(format!("{}", die1 + die2 + stat.value)).await?;
    }

    Ok(())
}
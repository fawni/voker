use comfy::*;

use orbs::{Orb, Orbs};
use spells::Spell;

mod orbs;
mod spells;

simple_game!("voker", Invoker, setup, update);

macro_rules! resource {
    ($r:literal) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $r))
    };
}

struct Invoker {
    orbs: Orbs,
    first_spell: Option<Spell>,
    second_spell: Option<Spell>,
}

impl Invoker {
    fn new(_c: &EngineState) -> Self {
        Self {
            orbs: Orbs::new(),
            first_spell: None,
            second_spell: None,
        }
    }

    fn add_orb(&mut self, orb: orbs::Orb) {
        play_sound("Click");
        self.orbs.push(orb);
        warn!("{:?}", self.orbs);
    }

    fn invoke(&mut self) {
        play_sound("Invoke");
        if let Some(spell) = self.orbs.invoke() {
            if let Some(first) = self.first_spell {
                if first.eq(&spell) {
                    return;
                }
            }

            self.second_spell = self.first_spell;
            self.first_spell = Some(spell);
            info!("Invoked spell: {}", spell);
        }
    }

    fn cast_first_spell(&self) {
        if let Some(spell) = self.first_spell {
            play_sound(&spell.to_string());
        }
    }

    fn cast_second_spell(&self) {
        if let Some(spell) = self.second_spell {
            play_sound(&spell.to_string());
        }
    }
}

fn setup(_invoker: &mut Invoker, ctx: &mut EngineContext) {
    twink::log::setup();

    ctx.load_texture_from_bytes("Quas", resource!("orbs/quas.png"));
    ctx.load_texture_from_bytes("Wex", resource!("orbs/wex.png"));
    ctx.load_texture_from_bytes("Exort", resource!("orbs/exort.png"));

    ctx.load_texture_from_bytes("Alacrity", resource!("spells/alacrity.png"));
    ctx.load_texture_from_bytes("Chaos Meteor", resource!("spells/chaos_meteor.png"));
    ctx.load_texture_from_bytes("Cold Snap", resource!("spells/cold_snap.png"));
    ctx.load_texture_from_bytes("Deafening Blast", resource!("spells/deafening_blast.png"));
    ctx.load_texture_from_bytes("EMP", resource!("spells/emp.png"));
    ctx.load_texture_from_bytes("Forge Spirit", resource!("spells/forge_spirit.png"));
    ctx.load_texture_from_bytes("Ghost Walk", resource!("spells/ghost_walk.png"));
    ctx.load_texture_from_bytes("Ice Wall", resource!("spells/ice_wall.png"));
    ctx.load_texture_from_bytes("Sun Strike", resource!("spells/sun_strike.png"));
    ctx.load_texture_from_bytes("Tornado", resource!("spells/tornado.png"));

    ctx.load_texture_from_bytes("Invoke", resource!("spells/invoke.png"));

    set_master_volume(0.1);

    load_sound_from_bytes(
        "Music",
        resource!("sounds/music.ogg"),
        StaticSoundSettings::new().volume(0.4),
    );

    load_sound_from_bytes(
        &Spell::Alacrity.to_string(),
        resource!("sounds/alacrity.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::ChaosMeteor.to_string(),
        resource!("sounds/chaos_meteor.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::ColdSnap.to_string(),
        resource!("sounds/cold_snap.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::DeafeningBlast.to_string(),
        resource!("sounds/deafening_blast.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::EMP.to_string(),
        resource!("sounds/emp.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::ForgeSpirit.to_string(),
        resource!("sounds/forge_spirit.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::GhostWalk.to_string(),
        resource!("sounds/ghost_walk.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::IceWall.to_string(),
        resource!("sounds/ice_wall.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::SunStrike.to_string(),
        resource!("sounds/sun_strike.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );
    load_sound_from_bytes(
        &Spell::Tornado.to_string(),
        resource!("sounds/tornado.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );

    load_sound_from_bytes(
        "Click",
        resource!("sounds/click.ogg"),
        StaticSoundSettings::new().volume(0.4),
    );
    load_sound_from_bytes(
        "Invoke",
        resource!("sounds/invoke.ogg"),
        StaticSoundSettings::new().volume(0.6),
    );

    play_music("Music");
}

fn update(invoker: &mut Invoker, _ctx: &mut EngineContext) {
    clear_background(Color::new(0.01, 0.01, 0.01, 1.0));

    draw_sprite(texture_id("Quas"), vec2(-8.0, -5.0), WHITE, 0, splat(3.0));
    draw_sprite(texture_id("Wex"), vec2(-4.0, -5.0), WHITE, 0, splat(3.0));
    draw_sprite(texture_id("Exort"), vec2(0.0, -5.0), WHITE, 0, splat(3.0));

    if is_key_pressed(KeyCode::Q) {
        invoker.add_orb(Orb::Quas);
    }

    if is_key_pressed(KeyCode::W) {
        invoker.add_orb(Orb::Wex);
    }

    if is_key_pressed(KeyCode::E) {
        invoker.add_orb(Orb::Exort);
    }

    if is_key_pressed(KeyCode::D) {
        invoker.cast_first_spell();
    }

    if is_key_pressed(KeyCode::F) {
        invoker.cast_second_spell();
    }

    if is_key_pressed(KeyCode::R) {
        invoker.invoke();
    }

    if let Some(spell) = invoker.first_spell {
        draw_sprite(
            texture_id(&spell.to_string()),
            vec2(4.0, -5.0),
            WHITE,
            0,
            splat(3.0),
        );
    }

    if let Some(spell) = invoker.second_spell {
        draw_sprite(
            texture_id(&spell.to_string()),
            vec2(8.0, -5.0),
            WHITE,
            0,
            splat(3.0),
        );
    }
}

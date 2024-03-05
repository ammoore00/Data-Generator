// Documentation comments sourced from the Minecraft Wiki (https://minecraft.wiki)

class BiomeElement extends NamespacedDataElement {
    //Determines whether or not the biome has precipitation.
    has_precipitation: boolean
    // Controls gameplay features like grass and foliage color, and a height adjusted temperature
    // (which controls whether raining or snowing if  has_precipitation is true, and generation 
    // details of some features).
    temperature: number
    // (optional, defaults to none) Either none or frozen. Modifies temperature before calculating
    // the height adjusted temperature. If frozen, makes some places' temperature high enough to rain (0.2).
    temperature_modifier?: TemperatureModifier
    downfall: number
    effects: BiomeEffect

    serialize(): object {
        throw new Error("Method not implemented.");
    }

    deserialize(json: object): void {
        throw new Error("Method not implemented.");
    }
}

interface BiomeEffect {
    fog_color: number
    sky_color: number
    water_color: number
    water_fog_color: number
    foliage_color?: number
    grass_color?: number
    grass_color_modifier?: GrassColorModifier
    particle?: BiomeParticle
    ambient_sound: Sound
    mood_sound?: {
        sound: Sound,
        tick_delay: number,
        block_search_extent: number,
        offset: number
    }
}

enum TemperatureModifier{
    NONE = "none",
    FROZEN = "frozen"
}

enum GrassColorModifier{
    NONE = "none",
    DARK_FOREST = "dark_forest",
    SWAMP = "swamp"
}

type Sound = string | {sound_id: string, range?: number}

interface BiomeParticle {

}
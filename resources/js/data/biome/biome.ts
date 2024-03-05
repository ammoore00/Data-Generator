interface biome {
    name: string,
    resource_location: string,

    has_precipiatation: boolean,
    temperature: number,
    temperature_modifier: "none | frozen"
    downfall: number,
    effects: biome_effect
}

interface biome_effect {
    fog_color: number,
    sky_color: number,
    water_color: number,
    water_fog_color: number,
    foliage_color: number,
    grass_color: number,
    grass_color_modifier: "none | dark_forest | swamp",
    //particle
}
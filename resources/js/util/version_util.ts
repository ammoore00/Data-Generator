class DatapackFormat {
    minVersion: number[]
    maxVersion: number[]
    format: number
}

const DatapackFormats = {
    FORMAT_26: {minVersion: [20, 3], maxVersion: [20, 4], format: 26},
    FORMAT_18: {minVersion: [20, 1], maxVersion: [20, 2], format: 18}
}
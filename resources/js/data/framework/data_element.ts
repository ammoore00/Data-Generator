abstract class DataElement {
    name: string
    properties: ObjectDataProperty

    constructor(name: string) {
        this.name = name
        this.populateProperties()
    }

    abstract populateProperties(): void

    abstract serialize(format: DatapackFormat): object
    abstract deserialize(json:object): void
}

abstract class NamespacedDataElement extends DataElement {
    resourceLocation: string

    constructor(resourceLocation: string, displayName: string) {
        super(displayName)

        if (validateResourceLocation(resourceLocation)) {
            this.resourceLocation = resourceLocation
        }
        else {
            throw new Error("Invalid format for resource location " + resourceLocation)
        }
    }
}
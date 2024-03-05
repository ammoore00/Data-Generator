abstract class DataElement {
    name: string
    properties: CompoundDataProperty

    constructor(name: string) {
        this.name = name
    }

    abstract serialize(): object
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
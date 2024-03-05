abstract class DataProperty<T> {
    displayName: string
    name: string
    value: T
    
    constructor(displayName: string, name: string, defaultValue: T) {
        this.displayName = displayName
        this.name = name
        this.value = defaultValue
    }

    abstract serialize(format: DatapackFormat): any
}

abstract class BasicDataProperty<T> extends DataProperty<T> {
    serialize(format: DatapackFormat): any {
        return this.value
    }
}

abstract class ElementDataProperty<T extends DataElement> extends DataProperty<T> {
    serialize(format: DatapackFormat): any {
        return this.value.serialize
    }
}

abstract class VersionedDataProperty extends DataProperty<any> {
    protected propertyMap: Map<(format: DatapackFormat) => boolean, DataProperty<any>>

    addProperty(condition: (format: DatapackFormat) => boolean, dataProperty: DataProperty<any>): void {
        this.propertyMap.set(condition, dataProperty)
    }

    serialize(format: DatapackFormat): any {
        for (let [condition, dataProperty] of this.propertyMap) {
            if (condition.apply(format)) {
                return dataProperty.serialize(format)
            }
        }

        throw new Error("Format " + format + " does not have any valid data properties assigned for " + this.name)
    }
}
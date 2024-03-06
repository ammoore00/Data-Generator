abstract class DataProperty<T> {
    name?: string

    constructor(name?: string) {
        this.name = name
    }

    abstract getValue(): T
    abstract serialize(format: DatapackFormat): any
}

abstract class StandardDataProperty<T> extends DataProperty<T> {
    value: T

    constructor(name: string = "", defaultValue: T) {
        super(name)
        this.value = defaultValue
    }

    getValue(): T {
        return this.value
    }
}

class SimpleDataProperty<T> extends StandardDataProperty<T> {
    serialize(format: DatapackFormat): any {
        return this.name + ":" + this.value
    }
}

class ElementDataProperty<T extends DataElement> extends StandardDataProperty<T> {
    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name != "") {
            output = this.name + ":"
        }

        output += this.value.serialize(format)
        return output
    }
}

class ObjectDataProperty extends DataProperty<DataProperty<any>[]> {
    properties: DataProperty<any>[]

    constructor(name?: string) {
        super(name)
        this.properties = new Array<DataProperty<any>>
    }

    addProperty(property: DataProperty<any>): void {
        this.properties.push(property)
    }

    getValue(): DataProperty<any>[] {
        throw this.properties
    }

    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name) {
            output = this.name + ":"
        }

        output += "{\n"

        for (let property of this.properties) {
            if (property.name == "") {
                throw new Error("Sub-properties of object data property cannot be anonymous!")
            }
            
            output += property.serialize + "\n"
        }

        output += "}"

        return output
    }
}

class ArrayDataProperty<T> extends DataProperty<T[]> {
    entries: T[]

    constructor(name?: string) {
        super(name)
        this.entries = new Array<T>
    }

    addEntry(entry: T): void {
        this.entries.push(entry)
    }

    getValue(): T[] {
        throw this.entries
    }

    serialize(format: DatapackFormat): any {
        let output: string = ""

        if (this.name != "") {
            output = this.name + ":"
        }

        output += "[\n"

        for (let entry of this.entries) {
            output += this.serlializeEntry(entry, format) + ",\n"
        }

        output += "]"

        return output
    }

    serlializeEntry(entry: T, format: DatapackFormat): any {
        return "" + entry
    }
}

class SerializableArrayDataProperty<T extends Serializable> extends ArrayDataProperty<T> {
    serlializeEntry(entry: T, format: DatapackFormat): any {
        if (entry instanceof DataProperty && entry.name != "") {
            throw new Error("Sub-properties of array data property cannot be named!")
        }

        return entry.serialize(format)
    }
}

class VersionedDataProperty extends DataProperty<any> {
    protected propertyMap: Map<(format: DatapackFormat) => boolean, DataProperty<any>>

    addProperty(condition: (format: DatapackFormat) => boolean, dataProperty: DataProperty<any>): void {
        this.propertyMap.set(condition, dataProperty)
    }

    getValue() {
        throw new Error("Method not implemented.")
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
abstract class DataProperty<T> {
    name: string
    value: T

    constructor(name: string = "", defaultValue: T) {
        this.name = name
        this.value = defaultValue
    }

    abstract serialize(format: DatapackFormat): string
}

class BasicDataProperty<T> extends DataProperty<T> {
    serialize(format: DatapackFormat): string {
        return this.name + ":" + this.value
    }
}

class ElementDataProperty<T extends DataElement> extends DataProperty<T> {
    serialize(format: DatapackFormat): string {
        let output: string = ""

        if (this.name.length > 0) {
            output = this.name + ":"
        }

        output += this.value.serialize()
        return output
    }
}

class CompoundDataProperty extends DataProperty<Array<any>> {
    isArray: boolean

    constructor(name: string = "", isArray: boolean) {
        super(name, [])
        this.isArray = isArray
    }

    serialize(format: DatapackFormat): string {
        let output: string = ""

        if (this.name != "") {
            output = this.name + ":"
        }

        if (this.isArray) {
            output += "[\n"

            for (let property of this.value) {
                if (property.name != "") {
                    throw new Error("Sub-properties of array data property cannot be named!")
                }
                
                output += property.serialize + "\n"
            }
    
            output += "]"
        }
        else {
            output += "{\n"

            for (let property of this.value) {
                if (property.name == "") {
                    throw new Error("Sub-properties of object data property cannot be anonymous!")
                }
                
                output += property.serialize + "\n"
            }
    
            output += "}"
        }

        return output
    }
}

class VersionedDataProperty extends DataProperty<any> {
    protected propertyMap: Map<(format: DatapackFormat) => boolean, DataProperty<any>>

    constructor(name: string = "") {
        super(name, [])
    }

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
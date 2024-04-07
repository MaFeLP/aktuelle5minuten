/* tslint:disable */
/* eslint-disable */
/**
 * Aktuelle 5 Minuten
 * Aktuelle 5 Minuten API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * Status about how many articles/categories exist in the database
 * @export
 * @interface Count
 */
export interface Count {
    /**
     * 
     * @type {number}
     * @memberof Count
     */
    articles: number;
    /**
     * 
     * @type {number}
     * @memberof Count
     */
    categories: number;
}

/**
 * Check if a given object implements the Count interface.
 */
export function instanceOfCount(value: object): boolean {
    if (!('articles' in value)) return false;
    if (!('categories' in value)) return false;
    return true;
}

export function CountFromJSON(json: any): Count {
    return CountFromJSONTyped(json, false);
}

export function CountFromJSONTyped(json: any, ignoreDiscriminator: boolean): Count {
    if (json == null) {
        return json;
    }
    return {
        
        'articles': json['articles'],
        'categories': json['categories'],
    };
}

export function CountToJSON(value?: Count | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'articles': value['articles'],
        'categories': value['categories'],
    };
}

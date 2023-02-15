using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

//-ENUM-S-
public enum eEnemyId 
{ 
    NONE = 0,
    ID_0 = 1,
    ID_1 = 2,
    ID_2 = 3,
    ID_3 = 4,
    ID_4 = 5,
    ID_5 = 6,
    ID_6 = 7,
    ID_7 = 8 
}
//-ENUM-E-

public class EnemyDef // ** AUTO-GENERATED CLASS FROM EXCEL SHEET, DO NOT EDIT NAME**
{
    //-DEF-FIELDS-S-
    public eEnemyId Id;
    public bool Enabled;
    public string stringyField;
    public float someFloatval;
    public RewardType rewardType;
    public List<int> someInts;
    public List<string> someStrings;
    public List<float> someFloats;
    public List<RewardType> someRewardTypes;
    public List<bool> someBools;
    public Reward reward1;
    public DateTime someDateTime;
    public Vector2 someVec2;
    public Vector3 someVec3;
    public Vector4 someVec4;
    //-DEF-FIELDS-E-
}

public static class Sheet1 // ** AUTO-GENERATED CLASS FROM EXCEL SHEET, DO NOT EDIT NAME**
{
    //-FIELDS-S-
    public const int someConstInt = 1;
    public static string someString = "some long string we \" want for some reason \nnextLine";
    public static float someVarFloat = 69.9f;
    public const bool someBoolVal = true;
    public enum SomeCustomEnum 
    {
       NONE = 0,
       Enum_1 = 1,
       Enum_2 = 2,
       Enum_3 = 3,
       Enum_4 = 4,
       Enum_5 = 5    
    };
    public const DateTime todaysDate = new DateTime(year:2020, month:4, day:3, hour:13, minute:0, second:0, kind:DateTimeKind.Utc); // Fri Apr  3 13:00:00;
    public const RewardType defaultReward = RewardType.gems;
    public const Vector2 someVec2 = new Vector2(22.3f, 40.6f);
    public const Vector3 someVec3 = new Vector3(21.3f, 44.6f, 745.2f);
    public const Vector4 someVec4 = new Vector4(27.3f, 440.6f, 3847f, 0f);
     
    // -- Everything below is a list of some sort. -- 
    public static List<int> someList = new List<int>() { 1, 2, 3, 4, 5 };
    public static List<float> chances = new List<float>() { 69.69f, 26.7f, 30.2f, 26.2f, 69.2f };
    public static List<string> tokens = new List<string>() { "Blaw_0", "Blaw_1", "Blaw_2", "Blaw_3", "Blaw_4" };
    public static List<Vector2> someVec2s = new List<Vector2>() { 
        new Vector2(22.3f, 40.6f), 
        new Vector2(22.3f, 40.6f), 
        new Vector2(22.3f, 40.6f) 
    };
    public static List<Vector3> someVec3s = new List<Vector3>() { 
        new Vector3(21.3f, 44.6f, 745.2f), 
        new Vector3(21.3f, 44.6f, 745.2f), 
        new Vector3(21.3f, 44.6f, 745.2f) 
    };
    public static List<Vector4> someVec4s = new List<Vector4>() { 
        new Vector4(27.3f, 440.6f, 3847f, 0f), 
        new Vector4(27.3f, 440.6f, 3847f, 0f), 
        new Vector4(27.3f, 440.6f, 3847f, 0f) 
    };
    public static List<DateTime> someDates = new List<DateTime>() { 
        new DateTime(year:2020, month:4, day:3, hour:13, minute:0, second:0, kind:DateTimeKind.Utc), // Fri Apr  3 13:00:00, 
        new DateTime(year:2020, month:5, day:3, hour:13, minute:0, second:0, kind:DateTimeKind.Utc), // Sun May  3 13:00:00, 
        new DateTime(year:2020, month:6, day:3, hour:13, minute:0, second:0, kind:DateTimeKind.Utc), // Wed Jun  3 13:00:00 
    };
    //-FIELDS-E-

    //-DEFS-S-
    public const int COUNT = 8; // Note: All except the NONE
    public static EnemyDef[] defs = new EnemyDef[]
    {
        null, // INDEX [0] / "NONE" IN THE ID ENUM
        
        new EnemyDef() // INDEX [1]
        {
            Id = eEnemyId.ID_0,
            Enabled = true,
            stringyField = "blaw
another Line
some " char",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                
            },
            someStrings = new List<string>()
            { 
                "natta",
            },
            someFloats = new List<float>()
            { 
                1.3f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.gold,
                RewardType.gold,
                RewardType.gold,
            },
            someBools = new List<bool>()
            { 
                true,
            },
            reward1 = new Reward()
            { 
                defId = 0, 
                name = "someName1",
                chance = 0.3f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:13, minute:0, second:0, kind:DateTimeKind.Utc), // Fri Apr  3 13:00:00
            someVec2 = new Vector2(22.3f, 40.6f),
            someVec3 = new Vector3(21.3f, 44.6f, 745.2f),
            someVec4 = new Vector4(27.3f, 440.6f, 3847f, 0f),
        },

        new EnemyDef() // INDEX [2]
        {
            Id = eEnemyId.ID_1,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                5,
            },
            someStrings = new List<string>()
            { 
                "something",
            },
            someFloats = new List<float>()
            { 
                1.2f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.gem,
                RewardType.gem,
                RewardType.gem,
            },
            someBools = new List<bool>()
            { 
                false,
            },
            reward1 = new Reward()
            { 
                defId = 1, 
                name = "someName2",
                chance = 0.2f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:13, minute:0, second:0, kind:DateTimeKind.Utc), // Fri Apr  3 13:00:00
            someVec2 = new Vector2(22.3f, 20.7f),
            someVec3 = new Vector3(21.3f, 41.6f, 745.3f),
            someVec4 = new Vector4(27.3f, 460.6f, 3847f, 1f),
        },

        new EnemyDef() // INDEX [3]
        {
            Id = eEnemyId.ID_2,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                67,
                76,
            },
            someStrings = new List<string>()
            { 
                "someother",
            },
            someFloats = new List<float>()
            { 
                69.9f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.coal,
                RewardType.coal,
                RewardType.coal,
            },
            someBools = new List<bool>()
            { 
                true,
            },
            reward1 = new Reward()
            { 
                defId = 2, 
                name = "someName3",
                chance = 0.4f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:1, minute:0, second:0, kind:DateTimeKind.Utc), // Fri Apr  3 01:00:00
            someVec2 = new Vector2(22.3f, 40.7f),
            someVec3 = new Vector3(21.3f, 44.6f, 745.3f),
            someVec4 = new Vector4(27.3f, 440.6f, 3847f, 1f),
        },

        new EnemyDef() // INDEX [4]
        {
            Id = eEnemyId.ID_3,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                
            },
            someStrings = new List<string>()
            { 
                "natta",
            },
            someFloats = new List<float>()
            { 
                92.7333333333333f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.gold,
                RewardType.gold,
                RewardType.gold,
            },
            someBools = new List<bool>()
            { 
                true,
            },
            reward1 = new Reward()
            { 
                defId = 3, 
                name = "someName4",
                chance = 0.4f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:0, minute:0, second:1, kind:DateTimeKind.Utc), // Fri Apr  3 00:00:01
            someVec2 = new Vector2(22.3f, 20.8f),
            someVec3 = new Vector3(21.3f, 41.6f, 745.4f),
            someVec4 = new Vector4(27.3f, 460.6f, 3847f, 2f),
        },

        new EnemyDef() // INDEX [5]
        {
            Id = eEnemyId.ID_4,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                129,
            },
            someStrings = new List<string>()
            { 
                "something",
            },
            someFloats = new List<float>()
            { 
                127.033333333333f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.gem,
                RewardType.gem,
                RewardType.gem,
            },
            someBools = new List<bool>()
            { 
                false,
            },
            reward1 = new Reward()
            { 
                defId = 4, 
                name = "someName5",
                chance = 0.45f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:0, minute:0, second:1, kind:DateTimeKind.Utc), // Fri Apr  3 00:00:01
            someVec2 = new Vector2(22.3f, 40.8f),
            someVec3 = new Vector3(21.3f, 44.6f, 745.4f),
            someVec4 = new Vector4(27.3f, 440.6f, 3847f, 2f),
        },

        new EnemyDef() // INDEX [6]
        {
            Id = eEnemyId.ID_5,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                191,
                76,
            },
            someStrings = new List<string>()
            { 
                "someother",
            },
            someFloats = new List<float>()
            { 
                161.333333333333f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.coal,
                RewardType.coal,
                RewardType.coal,
            },
            someBools = new List<bool>()
            { 
                true,
            },
            reward1 = new Reward()
            { 
                defId = 5, 
                name = "someName6",
                chance = 0.5f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:0, minute:0, second:1, kind:DateTimeKind.Utc), // Fri Apr  3 00:00:01
            someVec2 = new Vector2(22.3f, 20.9f),
            someVec3 = new Vector3(21.3f, 41.6f, 745.5f),
            someVec4 = new Vector4(27.3f, 460.6f, 3847f, 3f),
        },

        new EnemyDef() // INDEX [7]
        {
            Id = eEnemyId.ID_6,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                
            },
            someStrings = new List<string>()
            { 
                "natta",
            },
            someFloats = new List<float>()
            { 
                195.633333333333f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.gold,
                RewardType.gold,
                RewardType.gold,
            },
            someBools = new List<bool>()
            { 
                true,
            },
            reward1 = new Reward()
            { 
                defId = 6, 
                name = "someName7",
                chance = 0.55f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:0, minute:0, second:1, kind:DateTimeKind.Utc), // Fri Apr  3 00:00:01
            someVec2 = new Vector2(22.3f, 40.9f),
            someVec3 = new Vector3(21.3f, 44.6f, 745.5f),
            someVec4 = new Vector4(27.3f, 440.6f, 3847f, 3f),
        },

        new EnemyDef() // INDEX [8]
        {
            Id = eEnemyId.ID_7,
            Enabled = true,
            stringyField = "blaw",
            someFloatval = 34.3f,
            rewardType = RewardType.gold,
            someInts = new List<int>()
            { 
                253,
            },
            someStrings = new List<string>()
            { 
                "something",
            },
            someFloats = new List<float>()
            { 
                229.933333333333f,
            },
            someRewardTypes = new List<RewardType>()
            { 
                RewardType.gem,
                RewardType.gem,
                RewardType.gem,
            },
            someBools = new List<bool>()
            { 
                false,
            },
            reward1 = new Reward()
            { 
                defId = 7, 
                name = "someName8",
                chance = 0.6f,
            },
            someDateTime = new DateTime(year:2020, month:4, day:3, hour:0, minute:0, second:1, kind:DateTimeKind.Utc), // Fri Apr  3 00:00:01
            someVec2 = new Vector2(22.3f, 20.10f),
            someVec3 = new Vector3(21.3f, 41.6f, 745.6f),
            someVec4 = new Vector4(27.3f, 460.6f, 3847f, 4f),
        },
    };
    //-DEFS-E-

    //-FUNCTIONS-S-
    public static EnemyDef GetDef(eEnemyId id)
    {
        EnemyDef ret = null;
        int index = (int)id;
        if(index > -1 && index < defs.Length)
        {
            ret = defs[index];
        }
        else
        {
            Debug.LogError($"EnemyDef GetDef({index}) not found");
        }
        return ret;
    }
    
    //-FUNCTIONS-E-
}
    
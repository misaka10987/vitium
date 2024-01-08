#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const char *CAge;

/**
 * rust `std::Vec<T>`'s C-equivalant.
 *
 * Note that type infomation `T` will be lost since it is converted to C's pointer
 * therefore it is wise to export `T`'s C-equivalant and documentation as well.
 * Also, it is not guaranteed here that `T` correctly implements trait `C`.
 */
typedef struct CVector_CAge {
  CAge *head;
  uintptr_t len;
} CVector_CAge;

typedef const char *CDice;

typedef struct C_CAge_Cu64 {
  CAge v1;
  uint64_t v2;
} C_CAge_Cu64;

/**
 * rust `std::Vec<T>`'s C-equivalant.
 *
 * Note that type infomation `T` will be lost since it is converted to C's pointer
 * therefore it is wise to export `T`'s C-equivalant and documentation as well.
 * Also, it is not guaranteed here that `T` correctly implements trait `C`.
 */
typedef struct CVector_C_CAge_Cu64 {
  struct C_CAge_Cu64 *head;
  uintptr_t len;
} CVector_C_CAge_Cu64;

typedef struct CVector_C_CAge_Cu64 CPrice;

/**
 * Instance of weapon.
 */
typedef struct CWeapon {
  /**
   * Unique in-game id generated automatically. Set to `0` to let the program generate.
   */
  int64_t uid;
  /**
   * String ID for `Item`, must be unique.
   *
   * Any char that is allowed in a valid filename is allowed here, like `-`.
   */
  const char *id;
  /**
   * Name dieplayed in game.
   */
  const char *name;
  /**
   * Description displayed in game.
   */
  const char *descr;
  /**
   * Age periods available.
   */
  struct CVector_CAge age;
  /**
   * Damage expression using dice, eg `1d4+1`.
   */
  CDice atk;
  /**
   * In milimetres, `0` for melee weapons.
   */
  uint32_t rng;
  /**
   * Whether to apply penetration.
   */
  bool pntr;
  /**
   * Number of attacks able to inflict in a turn.
   */
  uint8_t per_turn;
  /**
   * Charges remaining.
   */
  uint8_t charge;
  /**
   * Charges used per attack.
   */
  uint8_t load;
  /**
   * Price in different time periods.
   */
  CPrice price;
} CWeapon;

typedef const char *CBodyPart;

/**
 * rust `std::Vec<T>`'s C-equivalant.
 *
 * Note that type infomation `T` will be lost since it is converted to C's pointer
 * therefore it is wise to export `T`'s C-equivalant and documentation as well.
 * Also, it is not guaranteed here that `T` correctly implements trait `C`.
 */
typedef struct CVector_CBodyPart {
  CBodyPart *head;
  uintptr_t len;
} CVector_CBodyPart;

/**
 * Defines species for deciding if an armor is able to wear.
 */
typedef enum CSpecies_Tag {
  /**
   * Human-liked species.
   */
  Human,
  /**
   * Non human-liked species.
   */
  NonHuman,
  /**
   * Let host decide if able to wear.
   */
  Else,
} CSpecies_Tag;

typedef struct CSpecies {
  CSpecies_Tag tag;
  union {
    struct {
      const char *else_;
    };
  };
} CSpecies;

/**
 * Instance of armor.
 */
typedef struct CArmor {
  /**
   * Unique in-game id generated automatically. Set to `0` to let the program generate.
   */
  int64_t uid;
  /**
   * String ID for `Item`, must be unique.
   *
   * Any char that is allowed in a valid filename is allowed here, like `-`.
   */
  const char *id;
  /**
   * Name dieplayed in game.
   */
  const char *name;
  /**
   * Description displayed in game.
   */
  const char *descr;
  /**
   * Age periods available.
   */
  struct CVector_CAge age;
  /**
   * Damage
   */
  const char *def;
  /**
   * Covered body parts.
   */
  struct CVector_CBodyPart cover;
  /**
   * Species able to wear this armor.
   */
  struct CSpecies species;
  /**
   * Whether resists penetration.
   */
  bool rerist_pntr;
  /**
   * Price in different time periods.
   */
  CPrice price;
} CArmor;

typedef struct COtherItem {
  /**
   * Unique in-game id generated automatically. Set to `0` to let the program generate.
   */
  int64_t uid;
  /**
   * String ID for `Item`, must be unique.
   *
   * Any char that is allowed in a valid filename is allowed here, like `-`.
   */
  const char *id;
  /**
   * Name displayed in game.
   */
  const char *name;
  /**
   * Description displayed in game.
   */
  const char *descr;
} COtherItem;

typedef enum CItem_Tag {
  Weapon,
  Armor,
  Other,
} CItem_Tag;

typedef struct CItem {
  CItem_Tag tag;
  union {
    struct {
      struct CWeapon weapon;
    };
    struct {
      struct CArmor armor;
    };
    struct {
      struct COtherItem other;
    };
  };
} CItem;

const char *vitium_common_cfi_hello(void);

struct CItem c_obj_CItem(const char *json);

const char *c_json_CItem(struct CItem obj);

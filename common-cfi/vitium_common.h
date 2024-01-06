#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * rust `std::Vec<T>`'s C-equivalant.
 *
 * Note that type infomation `T` will be lost since it is converted to C's pointer
 * therefore it is wise to export `T`'s C-equivalant and documentation as well.
 * Also, it is not guaranteed here that `T` correctly implements trait `C`.
 */
typedef struct CVector {
  const int8_t *head;
  uintptr_t len;
} CVector;

typedef const char *CDice;

typedef struct CVector CPrice;

typedef struct CWeapon {
  int64_t uid;
  const char *id;
  const char *name;
  const char *descr;
  struct CVector age;
  CDice atk;
  uint32_t rng;
  bool pntr;
  uint8_t per_turn;
  uint8_t charge;
  uint8_t load;
  CPrice price;
} CWeapon;

typedef enum CSpecies_Tag {
  Human,
  NonHuman,
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
  struct CVector age;
  /**
   * Damage
   */
  const char *def;
  /**
   * Covered body parts.
   */
  struct CVector cover;
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
      const struct CWeapon *weapon;
    };
    struct {
      const struct CArmor *armor;
    };
    struct {
      const struct COtherItem *other;
    };
  };
} CItem;

const char *vitium_common_cfi_hello(void);

struct CItem c_obj_CItem(const char *json);

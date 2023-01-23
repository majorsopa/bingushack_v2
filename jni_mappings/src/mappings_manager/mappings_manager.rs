use std::collections::HashMap;

use jni::JNIEnv;

use super::class_mapping::ClassMapping;
use super::SigHolder;

#[derive(Debug, Default)]
pub struct MappingsManager<'a> {
    mappings: HashMap<&'static str, ClassMapping<'a>>
}

impl MappingsManager<'_> {
    pub fn new(env: JNIEnv<'static>) -> Self {
        macro_rules! adds {
            ($cm:ident) => {
                #[allow(unused_macros)]
                macro_rules! add_field {
                    ($key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {
                        let sig = SigHolder::new($ob_name, $sig);
                        $cm.add_field(
                            $key_name,
                            sig,
                            $is_static,
                        )
                    };
                }

                #[allow(unused_macros)]
                macro_rules! add_method {
                    ($key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {
                        let sig = SigHolder::new($ob_name, $sig);
                        $cm.add_method(
                            $key_name,
                            sig,
                            $is_static,
                        )
                    };
                }
            };
        }
        // macro for making a class mapping
        macro_rules! add_mapping {
            (
                $new_self:ident,
                $class_name:literal,            // the easy-to-use name of the class
                $class_path:literal,            // path to the class or the obfuscated class name
                $fields_and_methods:block       // the fields and methods of the class (using the `add_field_or_method!` macro)
            ) => {{
                #[allow(unused_mut)]
                let mut cm = ClassMapping::new_from_class(env.find_class($class_path).unwrap());

                adds!(cm);
                $fields_and_methods

                $new_self.mappings.insert($class_name, cm);
            }}
        }

        let mut new_self = MappingsManager::default();

        add_mapping!(new_self, "MinecraftClient", "ejf", {
            add_field!("player", "t", "Lfcz;", false);
            add_field!("inGameHud", "l", "Lekn;", false);
            add_field!("interactionManager", "r", "Lezd;", false);
            add_field!("crosshairTarget", "w", "Leac;", false);
            add_field!("world", "s", "Leyz;", false);
            add_field!("targetedEntity", "v", "Lbdr;", false);
            add_field!("currentScreen", "z", "Lepb;", false);
            add_field!("gameRenderer", "j", "Lfdo;", false);
            add_field!("options", "m", "Lejj;", false);

            add_method!("getTickDelta", "av", "()F", false);
            add_method!("doAttack", "bg", "()Z", false);

            add_method!("getInstance", "N", "()Lejf;", true);
        });
        add_mapping!(new_self, "PlayerEntity", "bwp", {
            add_field!("currentScreenHandler", "bU", "Lbzg;", false);

            add_method!("getInventory", "fE", "()Lbwo;", false);
            add_method!("getOffHandStack", "eG", "()Lcdt;", false);
            add_method!("displayClientMessage", "a", "(Lss;Z)V", false);
            add_method!("isUsingItem", "eZ", "()Z", false);
            add_method!("getAttackCooldownProgress", "w", "(F)F", false);
            add_method!("swingHand", "a", "(Lbcl;Z)V", false);
        });
        add_mapping!(new_self, "ItemStack", "cdt", {
            add_method!("getItem", "c", "()Lcdp;", false);
        });
        add_mapping!(new_self, "Item", "cdp", {
            add_method!("getRawId", "a", "(Lcdp;)I", true);
        });
        add_mapping!(new_self, "Items", "cdw", {
            add_field!("TOTEM_OF_UNDYING", "tV", "Lcdp;", true);
        });
        add_mapping!(new_self, "Inventory", "bwo", {
            add_field!("selectedSlot", "k", "I", false);

            add_method!("getStack", "a", "(I)Lcdt;", false);
        });
        add_mapping!(new_self, "InGameHud", "ekn", {
            add_method!("chatHud", "d", "()Lela;", false);
        });
        add_mapping!(new_self, "ChatHud", "ela", {
            add_method!("addMessage", "a", "(Lss;Ltd;ILejb;Z)V", false);  // Text message, @Nullable MessageSignatureData signature, int ticks, @Nullable MessageIndicator indicator, boolean refresh
        });
        add_mapping!(new_self, "Text", "ss", {
            add_method!("of", "a", "(Ljava/lang/String;)Lss;", true);
        });
        add_mapping!(new_self, "InteractionManager", "ezd", {
            add_method!("clickSlot", "a", "(IIILbzp;Lbwp;)V", false);
            add_method!("attackEntity", "a", "(Lbwp;Lbdr;)V", false);
        });
        add_mapping!(new_self, "ScreenHandler", "bzg", {
            add_field!("syncId", "j", "I", false);
        });
        add_mapping!(new_self, "SlotActionType", "bzp", {
            add_field!("PICKUP", "a", "Lbzp;", true);
            add_field!("SWAP", "c", "Lbzp;", true);
        });
        add_mapping!(new_self, "Optional", "java/util/Optional", {
            add_method!("isPresent", "isPresent", "()Z", false);
            add_method!("get", "get", "()Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "DebugRenderer", "fgz", {
            add_method!("getTargetedEntity", "a", "(Lbdr;I)Ljava/util/Optional;", true);
        });
        add_mapping!(new_self, "Entity", "bdr", {
            add_field!("hurtTime", "aK", "I", false);

            add_method!("isAlive", "br", "()Z", false);
        });
        add_mapping!(new_self, "HitResult", "eac", {
            add_method!("getType", "c", "()Leac$a;", false);
        });
        add_mapping!(new_self, "HitResultType", "eac$a", {
            add_field!("ENTITY", "c", "Leac$a;", true);
        });
        add_mapping!(new_self, "Hand", "bcl", {
            add_field!("MAIN_HAND", "a", "Lbcl;", true);
            add_field!("OFF_HAND", "b", "Lbcl;", true);
        });
        add_mapping!(new_self, "ClientWorld", "eyz", {
            add_method!("sendPacketToServer", "a", "(Luh;)V", false);
        });
        add_mapping!(new_self, "PlayerInteractEntityC2SPacket", "zi", {
            add_method!("attack", "a", "(Lbdr;Z)Lzi;", true);
        });
        add_mapping!(new_self, "Slot", "cax", {
            add_field!("index", "e", "I", false);

            add_method!("getStack", "e", "()Lcdt;", false);
        });
        add_mapping!(new_self, "HandledScreen", "epz", {
            add_field!("focusedSlot", "u", "Lcax;", false);
        });
        add_mapping!(new_self, "Screen", "epb", {

        });
        add_mapping!(new_self, "GameRenderer", "fdo", {
            add_field!("fovMultiplier", "p", "F", false);
        });
        add_mapping!(new_self, "SimpleOption", "eji", {
            add_field!("value", "k", "Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "GameOptions", "ejj", {
            add_field!("fov", "bH", "Leji;", false);
        });
        add_mapping!(new_self, "Integer", "java/lang/Integer", {
            add_method!("intValue", "intValue", "()I", false);

            add_method!("valueOf", "valueOf", "(I)Ljava/lang/Integer;", true);
        });
        add_mapping!(new_self, "Object", "java/lang/Object", {
            
        });
        add_mapping!(new_self, "Viewport", "com/mojang/blaze3d/platform/GlStateManager$m", {  // static enum
            add_field!("x", "b", "I", true);
            add_field!("y", "c", "I", true);
            add_field!("width", "d", "I", true);
            add_field!("height", "e", "I", true);
        });
        add_mapping!(new_self, "RenderSystem", "com/mojang/blaze3d/systems/RenderSystem", {
            add_method!("getModelViewMatrix", "getModelViewMatrix", "()Lorg/joml/Matrix4f;", true);
            add_method!("getProjectionMatrix", "getProjectionMatrix", "()Lorg/joml/Matrix4f;", true);
        });
        add_mapping!(new_self, "Matrix4f", "org/joml/Matrix4f", {
            // bruh
            add_field!("m00", "m00", "F", false);
            add_field!("m01", "m01", "F", false);
            add_field!("m02", "m02", "F", false);
            add_field!("m03", "m03", "F", false);

            add_field!("m10", "m10", "F", false);
            add_field!("m11", "m11", "F", false);
            add_field!("m12", "m12", "F", false);
            add_field!("m13", "m13", "F", false);

            add_field!("m20", "m20", "F", false);
            add_field!("m21", "m21", "F", false);
            add_field!("m22", "m22", "F", false);
            add_field!("m23", "m23", "F", false);

            add_field!("m30", "m30", "F", false);
            add_field!("m31", "m31", "F", false);
            add_field!("m32", "m32", "F", false);
            add_field!("m33", "m33", "F", false);
        });

        new_self
    }

    pub fn get(&self, name: &str) -> Option<&ClassMapping> {
        unsafe {
            self.mappings.get(name).map(|r| std::mem::transmute::<&ClassMapping, &ClassMapping>(r))  // i don't know why this transmute is legal but it is so cope
        }
    }
}

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
            add_field!("renderTickCounter", "R", "Lejt;", false);

            add_method!("getTickDelta", "av", "()F", false);
            add_method!("doAttack", "bg", "()Z", false);
            add_method!("calculateBoundingBox", "ao", "()Ldzz;", false);
            add_method!("getNetworkHandler", "I", "()Leza;", false);

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
            add_field!("lastRenderX", "M", "D", false);
            add_field!("lastRenderY", "N", "D", false);
            add_field!("lastRenderZ", "O", "D", false);

            add_method!("isAlive", "br", "()Z", false);
            add_method!("getPos", "dd", "()Leae;", false);
            add_method!("getId", "ah", "()I", false);
            add_method!("getStringUUID", "ct", "()Ljava/lang/String;", false);
        });
        add_mapping!(new_self, "HitResult", "eac", {
            add_method!("getType", "c", "()Leac$a;", false);
        });
        add_mapping!(new_self, "HitResultType", "eac$a", {
            add_field!("ENTITY", "c", "Leac$a;", true);
            add_field!("MISS", "a", "Leac$a;", true);
        });
        add_mapping!(new_self, "Hand", "bcl", {
            add_field!("MAIN_HAND", "a", "Lbcl;", true);
            add_field!("OFF_HAND", "b", "Lbcl;", true);
        });
        add_mapping!(new_self, "ClientWorld", "eyz", {
            add_method!("sendPacketToServer", "a", "(Luh;)V", false);
            add_method!("getEntities", "e", "()Ljava/lang/Iterable;", false);
            add_method!("getBlockState", "a_", "(Lgp;)Lcyt;", false);
            add_method!("raycastBlock", "a", "(Leae;Leae;Lgp;Leax;Lcyt;)Leaa;", false);  // might not work, is a method of BlockView which i think is a superclass of this?
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

            add_method!("getCamera", "m", "()Leir;", false);
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
            add_method!("getX", "a", "()I", true);
            add_method!("getY", "b", "()I", true);
            add_method!("getWidth", "c", "()I", true);
            add_method!("getHeight", "d", "()I", true);
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
        //add_mapping!(new_self, "EntityRenderer", "fip", {
        //    add_method!("getRenderOffset", "a", "(Lbdr;F)Leae;", false);  // Vec3d getPositionOffset(T entity, float tickDelta)
        //});
        add_mapping!(new_self, "Vec3d", "eae", {
            add_field!("x", "c", "D", false);
            add_field!("y", "d", "D", false);
            add_field!("z", "e", "D", false);

            add_method!("distanceToSqr", "c", "(DDD)D", false);
            add_method!("subtract", "a", "(DDD)Leae;", false);
            add_method!("getBlockShape", "a", "(Lcyt;Lcjc;Lgp;)Leax;", false);

            add_method!("<init>", "<init>", "(DDD)V", true);
        });
        add_mapping!(new_self, "RenderTickCounter", "ejt", {
            add_field!("partialTick", "a", "F", false);
        });
        add_mapping!(new_self, "Iterable", "java/lang/Iterable", {
            add_method!("iterator", "iterator", "()Ljava/util/Iterator;", false);
        });
        add_mapping!(new_self, "Iterator", "java/util/Iterator", {
            add_method!("hasNext", "hasNext", "()Z", false);
            add_method!("next", "next", "()Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "Box", "dzz", {
            add_field!("minX", "a", "D", false);
            add_field!("minY", "b", "D", false);
            add_field!("minZ", "c", "D", false);
            add_field!("maxX", "d", "D", false);
            add_field!("maxY", "e", "D", false);
            add_field!("maxZ", "f", "D", false);

            add_method!("offset", "c", "(Leae;)Ldzz;", false);
        });
        add_mapping!(new_self, "LivingEntity", "beg", {
            add_method!("getAttributeInstance", "a", "(Lbfe;)Lbff;", false);
            add_method!("hasStatusEffect", "a", "(Lbdi;)Z", false);
            add_method!("getStatusEffect", "b", "(Lbdi;)Lbdk;", false);
        });
        add_mapping!(new_self, "Camera", "eir", {
            add_field!("pitch", "j", "F", false);
            add_field!("yaw", "k", "F", false);
        });
        add_mapping!(new_self, "ClientPlayNetworkHandler", "eza", {
            add_method!("getPlayerListEntry", "a", "(Ljava/lang/String;)Leze;", false);
        });
        add_mapping!(new_self, "GameMode", "cjt", {
            add_field!("SURVIVAL", "a", "Lcjt;", true);
            add_field!("CREATIVE", "b", "Lcjt;", true);
            add_field!("ADVENTURE", "c", "Lcjt;", true);
            add_field!("SPECTATOR", "d", "Lcjt;", true);
        });
        add_mapping!(new_self, "PlayerListEntry", "eze", {
            add_method!("getGameMode", "e", "()Lcjt;", false);
        });
        add_mapping!(new_self, "EntityAttributes", "bfj", {
            add_field!("GENERIC_ARMOR_TOUGHNESS", "j", "Lbfe;", true);
        });
        add_mapping!(new_self, "EntityAttribute", "bfe", {

        });
        add_mapping!(new_self, "EntityAttributeInstance", "bff", {

        });
        add_mapping!(new_self, "Explosion", "cjo", {
            add_method!("<init>", "<init>", "(Lcjw;Lbdr;Lbcz;Lcjp;DDDFZLcjo$a;)V", true);
        });
        add_mapping!(new_self, "DestructionType", "cjo$a", {
            add_field!("DESTROY_WITH_DECAY", "c", "Lcjo$a;", true);
        });
        add_mapping!(new_self, "StatusEffects", "bdm", {
            add_field!("RESISTANCE", "k", "Lbdi;", true);
        });
        add_mapping!(new_self, "StatusEffectInstance", "bdk", {
            add_method!("getAmplifier", "d", "()I", false);
        });
        add_mapping!(new_self, "MathHelper", "aoc", {
            add_method!("lerp", "d", "(DDD)D", true);
            add_method!("sign", "k", "(D)I", true);
            add_method!("fractionalPart", "g", "(D)D", true);
        });
        add_mapping!(new_self, "RaycastContext", "cjf", {
            add_method!("<init>", "<init>", "<init>(Leae;Leae;Lcjf$a;Lcjf$b;Lbdr;)V", true);
            add_method!("getEnd", "a", "()Leae;", false);
            add_method!("getStart", "b", "()Leae;", false);
            add_method!("getBlockShape", "a", "(Lcyt;Lcjc;Lgp;)Leax;", false);
        });
        add_mapping!(new_self, "BlockHitResult", "eaa", {
            add_method!("getPos", "a", "()Lgp;", false);

            // for getType method cast it up to a HitResult i think
            add_method!("createMissed", "a", "(Leae;Lgv;Lgp;)Leaa;", true);
        });
        add_mapping!(new_self, "BlockPos", "gp", {
            add_method!("<init>", "<init>", "(Lhu;)V", true);

            add_method!("getX", "u", "()I", false);
            add_method!("getY", "v", "()I", false);
            add_method!("getZ", "w", "()I", false);
        });
        add_mapping!(new_self, "BlockView", "cjc", {

        });
        add_mapping!(new_self, "VoxelShape", "eax", {
            add_method!("raycast", "a", "(Leae;Leae;Lgp;)Leaa;", false);
        });
        add_mapping!(new_self, "VoxelShapes", "eau", {
            add_method!("empty", "a", "()Leax;", true);
        });
        add_mapping!(new_self, "Direction", "gv", {
            add_method!("getFacing", "a", "(FFF)Lgv;", true);
        });
        //add_mapping!(new_self, "Vec3i", "hu", {
        //    add_method!("<init>", "<init>", "(DDD)V", true);
        //});
        add_mapping!(new_self, "ShapeType", "cjf$a", {
            add_field!("COLLIDER", "a", "Lcjf$a;", true);
        });
        add_mapping!(new_self, "FluidHandling", "cjf$b", {
            add_field!("NONE", "a", "Lcjf$b;", true);
        });
        add_mapping!(new_self, "BlockState", "cyt", {  // extends AbstractBlockState
            add_method!("getBlock", "b", "()Lcmt;", false);
        });
        add_mapping!(new_self, "Block", "cmt", {
            add_method!("getBlastResistance", "e", "()F", false);
            add_method!("getDefaultState", "n", "()Lcyt;", false);
        });
        //add_mapping!(new_self, "AbstractBlockState", "cys$a", {
        //    add_method!("getBlock", "b", "()Lcmt;", false);
        //});
        add_mapping!(new_self, "Blocks", "cmu", {
            add_field!("OBSIDIAN", "ce", "Lcmt;", true);
            add_field!("AIR", "a", "Lcmt;", true);
        });
        add_mapping!(new_self, "MutableBlockPos", "gp$a", {
            add_method!("<init>", "<init>", "(DDD)V", true);
            add_method!("set", "d", "(III)Lgp$a;", false);
        });

        new_self
    }

    pub fn get(&self, name: &str) -> Option<&ClassMapping> {
        unsafe {
            self.mappings.get(name).map(|r| std::mem::transmute::<&ClassMapping, &ClassMapping>(r))  // i don't know why this transmute is legal but it is so cope
        }
    }
}

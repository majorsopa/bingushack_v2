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

        add_mapping!(new_self, "MinecraftClient", "enn", {
            add_field!("player", "t", "Lfiy;", false);
            add_field!("inGameHud", "l", "Leow;", false);
            add_field!("interactionManager", "r", "Lffa;", false);
            add_field!("crosshairTarget", "w", "Leeg;", false);
            add_field!("world", "s", "Lfew;", false);
            add_field!("targetedEntity", "v", "Lbfj;", false);
            add_field!("currentScreen", "z", "Leuq;", false);
            add_field!("gameRenderer", "j", "Lfjq;", false);
            add_field!("options", "m", "Lenr;", false);
            add_field!("renderTickCounter", "R", "Leoa;", false);

            add_method!("getTickDelta", "av", "()F", false);
            add_method!("doAttack", "bj", "()Z", false);
            add_method!("getNetworkHandler", "I", "()Lfex;", false);
            add_method!("handleBlockBreaking", "g", "(Z)V", false);
            add_method!("startAttack", "bj", "()Z", false);
            add_method!("continueAttack", "g", "(Z)V", false);

            add_method!("getInstance", "N", "()Lenn;", true);
        });
        add_mapping!(new_self, "PlayerEntity", "byo", {
            add_field!("currentScreenHandler", "bR", "Lcbf;", false);

            add_method!("getInventory", "fN", "()Lbyn;", false);
            add_method!("getOffHandStack", "eP", "()Lcfz;", false);
            add_method!("displayClientMessage", "a", "(Lsw;Z)V", false);
            add_method!("isUsingItem", "fi", "()Z", false);
            add_method!("getAttackCooldownProgress", "A", "(F)F", false);
            add_method!("swingHand", "a", "(Lbdw;Z)V", false);
        });
        add_mapping!(new_self, "ItemStack", "czf", {
            add_method!("getItem", "d", "()Lcfu;", false);
        });
        add_mapping!(new_self, "Item", "cfu", {
            add_method!("getRawId", "a", "(Lcfu;)I", true);
        });
        add_mapping!(new_self, "Items", "cgc", {
            add_field!("TOTEM_OF_UNDYING", "uz", "Lcfu;", true);
        });
        add_mapping!(new_self, "Inventory", "bdq", {
            add_field!("selectedSlot", "l", "I", false);

            add_method!("getStack", "a", "(I)Lcfz;", false);
        });
        add_mapping!(new_self, "InGameHud", "ekn", {
            add_method!("chatHud", "d", "()Lela;", false);
        });
        add_mapping!(new_self, "ChatHud", "eow", {
            add_method!("addMessage", "a", "(Lsw;Lth;ILeni;Z)V", false);  // Text message, @Nullable MessageSignatureData signature, int ticks, @Nullable MessageIndicator indicator, boolean refresh
        });
        add_mapping!(new_self, "Text", "sw", {
            add_method!("of", "a", "(Ljava/lang/String;)Lsw;", true);
        });
        add_mapping!(new_self, "InteractionManager", "ffa", {
            add_method!("clickSlot", "a", "(IIILcbo;Lbyo;)V", false);
            add_method!("attackEntity", "a", "(Lbyo;Lbfj;)V", false);
            add_method!("cancelBlockBreaking", "b", "()V", false);
        });
        add_mapping!(new_self, "ScreenHandler", "cbf", {
            add_field!("syncId", "j", "I", false);
        });
        add_mapping!(new_self, "SlotActionType", "cbo", {
            add_field!("PICKUP", "a", "Lcbo;", true);
            add_field!("QUICK_MOVE", "b", "Lcbo;", true);
            add_field!("SWAP", "c", "Lcbo;", true);
        });
        add_mapping!(new_self, "Optional", "java/util/Optional", {
            add_method!("isPresent", "isPresent", "()Z", false);
            add_method!("get", "get", "()Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "DebugRenderer", "fnd", {
            add_method!("getTargetedEntity", "a", "(Lbfj;I)Ljava/util/Optional;", true);
        });
        add_mapping!(new_self, "Entity", "bfj", {
            add_field!("hurtTime", "aL", "I", false);
            add_field!("lastRenderX", "ab", "D", false);
            add_field!("lastRenderY", "ac", "D", false);
            add_field!("lastRenderZ", "ad", "D", false);

            add_method!("isAlive", "bs", "()Z", false);
            add_method!("getPos", "dg", "()Leei;", false);
            add_method!("getId", "af", "()I", false);
            add_method!("getStringUUID", "cu", "()Ljava/lang/String;", false);
        });
        add_mapping!(new_self, "HitResult", "eeg", {
            add_method!("getType", "c", "()Leeg$a;", false);
        });
        add_mapping!(new_self, "HitResultType", "eeg$a", {
            add_field!("MISS", "a", "Leeg$a;", true);
            add_field!("ENTITY", "c", "Leeg$a;", true);
        });
        add_mapping!(new_self, "Hand", "bdw", {
            add_field!("MAIN_HAND", "a", "Lbdw;", true);
            add_field!("OFF_HAND", "b", "Lbdw;", true);
        });
        add_mapping!(new_self, "ClientWorld", "few", {
            add_field!("clientWorldProperties", "F", "Lfew$a;", false);

            add_method!("sendPacketToServer", "a", "(Luo;)V", false);
            add_method!("getEntities", "e", "()Ljava/lang/Iterable;", false);  // maybe getting wrong entities for trigger bot? try the non-iter one (todo!)
            add_method!("getBlockState", "a_", "(Lgu;)Ldcb;", false);
            add_method!("raycastBlock", "a", "(Leei;Leei;Lgu;Lefb;Ldcb;)Leee;", false);  // might not work, is a method of BlockView which i think is a superclass of this?
        });
        add_mapping!(new_self, "PlayerInteractEntityC2SPacket", "zt", {
            add_method!("attack", "a", "(Lbfj;Z)Lzt;", true);
        });
        add_mapping!(new_self, "Slot", "ccx", {
            add_field!("index", "e", "I", false);

            add_method!("getStack", "e", "()Lcfz;", false);
        });
        add_mapping!(new_self, "HandledScreen", "evp", {
            add_field!("focusedSlot", "r", "Lccx;", false);
        });
        add_mapping!(new_self, "Screen", "euq", {

        });
        add_mapping!(new_self, "GameRenderer", "fjq", {
            add_field!("fovMultiplier", "q", "F", false);

            add_method!("getCamera", "m", "()Lemz;", false);
        });
        add_mapping!(new_self, "SimpleOption", "eji", {
            add_field!("value", "k", "Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "GameOptions", "enr", {
            add_field!("fov", "bM", "Lenq;", false);
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
        add_mapping!(new_self, "Vec3d", "eei", {
            add_field!("x", "c", "D", false);
            add_field!("y", "d", "D", false);
            add_field!("z", "e", "D", false);

            add_method!("distanceToSqr", "c", "(DDD)D", false);
            add_method!("subtract", "a", "(DDD)Leei;", false);

            add_method!("<init>", "<init>", "(DDD)V", true);
        });
        add_mapping!(new_self, "RenderTickCounter", "eoa", {
            add_field!("partialTick", "a", "F", false);
        });
        add_mapping!(new_self, "Iterable", "java/lang/Iterable", {
            add_method!("iterator", "iterator", "()Ljava/util/Iterator;", false);
        });
        add_mapping!(new_self, "Iterator", "java/util/Iterator", {
            add_method!("hasNext", "hasNext", "()Z", false);
            add_method!("next", "next", "()Ljava/lang/Object;", false);
        });
        add_mapping!(new_self, "Box", "eed", {
            add_field!("minX", "a", "D", false);
            add_field!("minY", "b", "D", false);
            add_field!("minZ", "c", "D", false);
            add_field!("maxX", "d", "D", false);
            add_field!("maxY", "e", "D", false);
            add_field!("maxZ", "f", "D", false);

            add_method!("offset", "c", "(Leei;)Leed;", false);
        });
        add_mapping!(new_self, "LivingEntity", "bfz", {
            add_method!("getAttributeInstance", "a", "(Lbhb;)Lbhc;", false);
            add_method!("hasStatusEffect", "a", "(Lbey;)Z", false);
            add_method!("getStatusEffect", "b", "(Lbey;)Lbfa;", false);
            add_method!("getArmor", "eF", "()I", false);
            add_method!("getArmorItems", "bJ", "()Ljava/lang/Iterable;", false);
        });
        add_mapping!(new_self, "Camera", "emz", {
            add_field!("pitch", "j", "F", false);
            add_field!("yaw", "k", "F", false);
        });
        add_mapping!(new_self, "ClientPlayNetworkHandler", "fex", {
            add_method!("getPlayerListEntry", "a", "(Ljava/lang/String;)Lffb;", false);
        });
        add_mapping!(new_self, "GameMode", "cmj", {
            add_field!("SURVIVAL", "a", "Lcmj;", true);
            add_field!("CREATIVE", "b", "Lcmj;", true);
            add_field!("ADVENTURE", "c", "Lcmj;", true);
            add_field!("SPECTATOR", "d", "Lcmj;", true);
        });
        add_mapping!(new_self, "PlayerListEntry", "ffb", {
            add_method!("getGameMode", "e", "()Lcmj;", false);
        });
        add_mapping!(new_self, "EntityAttributes", "bhg", {
            add_field!("GENERIC_ARMOR_TOUGHNESS", "j", "Lbhb;", true);
        });
        add_mapping!(new_self, "EntityAttribute", "bhb", {

        });
        add_mapping!(new_self, "EntityAttributeInstance", "bhc", {
            add_method!("getValue", "f", "()D", false);
        });
        add_mapping!(new_self, "Explosion", "cme", {
            add_method!("getCausingEntity", "e", "()Lbfz;", false);

            add_method!("<init>", "<init>", "(Lcmm;Lbfj;Lben;Lcmf;DDDFZLcme$a;)V", true);  // world, Nullable entity, Nullable damageSource, Nullable explosionBehavior, x, y, z, power, createFire, destructionType
        });
        add_mapping!(new_self, "DestructionType", "cme$a", {
            add_field!("DESTROY_WITH_DECAY", "c", "Lcme$a;", true);
        });
        add_mapping!(new_self, "StatusEffects", "bfc", {
            add_field!("RESISTANCE", "k", "Lbey;", true);
        });
        add_mapping!(new_self, "StatusEffect", "bey", {
            
        });
        add_mapping!(new_self, "StatusEffectInstance", "bfa", {
            add_method!("getAmplifier", "e", "()I", false);
        });
        add_mapping!(new_self, "MathHelper", "apa", {
            add_method!("lerp", "d", "(DDD)D", true);
            add_method!("sign", "j", "(D)I", true);
            add_method!("fractionalPart", "e", "(D)D", true);
            // gotta add a floor method
        });
        add_mapping!(new_self, "RaycastContext", "clv", {
            add_method!("<init>", "<init>", "(Leei;Leei;Lclv$a;Lclv$b;Lbfj;)V", true);
            add_method!("getEnd", "a", "()Leei;", false);
            add_method!("getStart", "b", "()Leei;", false);
            add_method!("getBlockShape", "a", "(Ldcb;Lcls;Lgu;)Lefb;", false);
        });
        add_mapping!(new_self, "BlockHitResult", "eee", {
            add_method!("getPos", "a", "()Lgu;", false);

            // for getType method cast it up to a HitResult i think
            add_method!("createMissed", "a", "(Leei;Lha;Lgu;)Leee;", true);
        });
        add_mapping!(new_self, "BlockPos", "gu", {
            add_method!("<init>", "<init>", "(Lhz;)V", true);

            add_method!("getX", "u", "()I", false);
            add_method!("getY", "v", "()I", false);
            add_method!("getZ", "w", "()I", false);
        });
        add_mapping!(new_self, "BlockView", "cls", {

        });
        add_mapping!(new_self, "VoxelShape", "efb", {
            add_method!("raycast", "a", "(Leei;Leei;Lgu;)Leee;", false);
        });
        add_mapping!(new_self, "VoxelShapes", "eey", {
            add_method!("empty", "a", "()Lefb;", true);
        });
        add_mapping!(new_self, "Direction", "ha", {
            add_method!("getFacing", "a", "(FFF)Lha;", true);
        });
        //add_mapping!(new_self, "Vec3i", "hu", {
        //    add_method!("<init>", "<init>", "(DDD)V", true);
        //});
        add_mapping!(new_self, "ShapeType", "clv$a", {
            add_field!("COLLIDER", "a", "Lclv$a;", true);
        });
        add_mapping!(new_self, "FluidHandling", "clv$b", {
            add_field!("NONE", "a", "Lclv$b;", true);
        });
        add_mapping!(new_self, "BlockState", "dcb", {  // extends AbstractBlockState
            add_method!("getBlock", "b", "()Lcpn;", false);
        });
        add_mapping!(new_self, "Block", "cpn", {
            add_method!("getBlastResistance", "d", "()F", false);
            add_method!("getDefaultState", "n", "()Ldcb;", false);
        });
        //add_mapping!(new_self, "AbstractBlockState", "cys$a", {
        //    add_method!("getBlock", "b", "()Lcmt;", false);
        //});
        add_mapping!(new_self, "Blocks", "cpo", {
            add_field!("OBSIDIAN", "co", "Lcpn;", true);
            add_field!("AIR", "a", "Lcpn;", true);
        });
        add_mapping!(new_self, "MutableBlockPos", "gu$a", {
            add_method!("<init>", "<init>", "(DDD)V", true);
            add_method!("set", "d", "(III)Lgu$a;", false);  // i dont think this works
        });
        add_mapping!(new_self, "String", "java/lang/String", {
            
        });
        add_mapping!(new_self, "ClientWorldProperties", "few$a", {
            add_method!("getDifficulty", "s", "()Lbdu;", false);
        });
        add_mapping!(new_self, "Difficulty", "bdu", {
            add_field!("PEACEFUL", "a", "Lbdu;", true);
            add_field!("EASY", "b", "Lbdu;", true);
            add_field!("NORMAL", "c", "Lbdu;", true);
            add_field!("HARD", "d", "Lbdu;", true);
        });
        add_mapping!(new_self, "DamageUtil", "bej", {
            add_method!("getDamageLeft", "a", "(FFF)F", true);
        });
        add_mapping!(new_self, "EnchantmentHelper", "cki", {
            add_method!("getProtectionAmount", "a", "(Ljava/lang/Iterable;Lben;)I", true);
        });
        //add_mapping!(new_self, "DamageSource", "ben", {
        //    add_method!("setScaledWithDifficulty", "w", "()Lbcz;", false);
        //    add_method!("setExplosive", "d", "()Lbcz;", false);
        //
        //    add_method!("<init>", "<init>", "(Ljava/lang/String;)V", true);
        //});
        //add_mapping!(new_self, "EntityDamageSource", "bda", {
        //    add_method!("<init>", "<init>", "(Ljava/lang/String;Lbdr;)V", true);
        //});
        //add_mapping!(new_self, "DeathScreen", "eoc", {
        //    add_method!("init", "b", "()V", false);
        //});

        new_self
    }

    pub fn get(&self, name: &str) -> Option<&ClassMapping> {
        unsafe {
            self.mappings.get(name).map(|r| std::mem::transmute::<&ClassMapping, &ClassMapping>(r))  // i don't know why this transmute is legal but it is so cope
        }
    }
}

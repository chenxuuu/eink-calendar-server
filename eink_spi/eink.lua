--- 模块功能：SPI接口的FLASH功能测试.
-- 读取FLASH ID
-- @author openLuat
-- @module spi.testSpiFlash
-- @license MIT
-- @copyright openLuat
-- @release 2018.03.27

module(...,package.seeall)

require"utils"
require"pm"
pm.wake("eink")
require"pins"
require"misc"

pmd.ldoset(15,pmd.LDO_VMMC)

local result = spi.setup(spi.SPI_1,0,0,8,10000000,0)--初始化spi，
log.info("testSpi.init",result)

local busy = pins.setup(3,nil,pio.PULLUP)
local dc = pins.setup(18,1)
local res = pins.setup(19,1)

function EPD_W21_WriteCMD(cmd)
    dc(0)
    spi.send(spi.SPI_1,string.char(cmd))
end

function EPD_W21_WriteDATA(cmd)
    dc(1)
    spi.send(spi.SPI_1,string.char(cmd))
end

function EPD_W21_WriteDATA_S(cmd)
    dc(1)
    spi.send(spi.SPI_1,cmd)
end

function lcd_chkstatus()
    repeat
        EPD_W21_WriteCMD(0x71)
        sys.wait(100)
        log.info("lcd busy")
    until busy() == 1
end

function EPD_W21_Init()
    res(0)
    sys.wait(500)
    res(1)
    sys.wait(500)
end

function EPD_init()
    EPD_W21_Init() --Electronic paper IC reset

    EPD_W21_WriteCMD(0x06)--         //boost soft start
    EPD_W21_WriteDATA(0x17)--   //A
    EPD_W21_WriteDATA(0x17)--   //B
    EPD_W21_WriteDATA(0x17)--   //C

    EPD_W21_WriteCMD(0x04)--  //Power on
    lcd_chkstatus()-- //waiting for the electronic paper IC to release the idle signal

    EPD_W21_WriteCMD(0x00)--     //panel setting
    EPD_W21_WriteDATA(0x1f)--    //LUT from OTP
    EPD_W21_WriteDATA(0x0d)--    //VCOM to 0V
end

function EPD_init_4gray()
    EPD_W21_Init() --Electronic paper IC reset

    EPD_W21_WriteCMD(0x01)--OWER SETTING
    EPD_W21_WriteDATA(0x03)
    EPD_W21_WriteDATA(0x00)
    EPD_W21_WriteDATA(0x2b)
    EPD_W21_WriteDATA(0x2b)
    EPD_W21_WriteDATA(0x13)

    EPD_W21_WriteCMD(0x06)--         //boost soft start
    EPD_W21_WriteDATA(0x17)--   //A
    EPD_W21_WriteDATA(0x17)--   //B
    EPD_W21_WriteDATA(0x17)--   //C

    EPD_W21_WriteCMD(0x04)--  //Power on
    lcd_chkstatus()-- //waiting for the electronic paper IC to release the idle signal

    EPD_W21_WriteCMD(0x00)--panel setting
    EPD_W21_WriteDATA(0x3f)--KW-3f   KWR-2F  BWROTP 0f BWOTP 1f

    EPD_W21_WriteCMD(0x30)--PLL setting
    EPD_W21_WriteDATA (0x3c)--100hz

    EPD_W21_WriteCMD(0x61)--resolution setting
    EPD_W21_WriteDATA(0x01)--400
    EPD_W21_WriteDATA(0x90)
    EPD_W21_WriteDATA(0x01)--300
    EPD_W21_WriteDATA(0x2c)

    EPD_W21_WriteCMD(0x82)--vcom_DC setting
    EPD_W21_WriteDATA (0x12)

    EPD_W21_WriteCMD(0X50)--VCOM AND DATA INTERVAL SETTING
    EPD_W21_WriteDATA(0x97)
end

function PIC_display1()
    EPD_W21_WriteCMD(0x10)--Transfer old data
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0)--
    end
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0)--
    end
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0xff)--
    end
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0xff)--
    end
    EPD_W21_WriteCMD(0x13)
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0)--
    end
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0xff)--
    end
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0)--
    end
    for _ = 1, 3750 do
        EPD_W21_WriteDATA(0xff)--
    end
end

local white = string.rep(string.char(0xff),15000)
function EPD_display_Clean()
    EPD_W21_WriteCMD(0x10)--Transfer old data
    EPD_W21_WriteDATA_S(white)
    EPD_W21_WriteCMD(0x13)--Transfer new data
    EPD_W21_WriteDATA_S(white)
end

--4gray
local lut_vcom =
{
0x00  ,0x0A ,0x00 ,0x00 ,0x00 ,0x01,
0x60  ,0x14 ,0x14 ,0x00 ,0x00 ,0x01,
0x00  ,0x14 ,0x00 ,0x00 ,0x00 ,0x01,
0x00  ,0x13 ,0x0A ,0x01 ,0x00 ,0x01,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00

};
--R21
local lut_ww ={
0x40  ,0x0A ,0x00 ,0x00 ,0x00 ,0x01,
0x90  ,0x14 ,0x14 ,0x00 ,0x00 ,0x01,
0x10  ,0x14 ,0x0A ,0x00 ,0x00 ,0x01,
0xA0  ,0x13 ,0x01 ,0x00 ,0x00 ,0x01,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
};
--R22H  r
local lut_bw ={
0x40  ,0x0A ,0x00 ,0x00 ,0x00 ,0x01,
0x90  ,0x14 ,0x14 ,0x00 ,0x00 ,0x01,
0x00  ,0x14 ,0x0A ,0x00 ,0x00 ,0x01,
0x99  ,0x0C ,0x01 ,0x03 ,0x04 ,0x01,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
};
--R23H  w
local lut_wb ={
0x40  ,0x0A ,0x00 ,0x00 ,0x00 ,0x01,
0x90  ,0x14 ,0x14 ,0x00 ,0x00 ,0x01,
0x00  ,0x14 ,0x0A ,0x00 ,0x00 ,0x01,
0x99  ,0x0B ,0x04 ,0x04 ,0x01 ,0x01,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
};
--R24H  b
local lut_bb ={
0x80  ,0x0A ,0x00 ,0x00 ,0x00 ,0x01,
0x90  ,0x14 ,0x14 ,0x00 ,0x00 ,0x01,
0x20  ,0x14 ,0x0A ,0x00 ,0x00 ,0x01,
0x50  ,0x13 ,0x01 ,0x00 ,0x00 ,0x01,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
0x00  ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
};
function EPD_refresh_4gray()
    EPD_W21_WriteCMD(0x20)
    for i=1,#lut_vcom do
        EPD_W21_WriteDATA(lut_vcom[i])
    end
    EPD_W21_WriteCMD(0x21)
    for i=1,#lut_vcom do
        EPD_W21_WriteDATA(lut_ww[i])
    end
    EPD_W21_WriteCMD(0x22)
    for i=1,#lut_vcom do
        EPD_W21_WriteDATA(lut_bw[i])
    end
    EPD_W21_WriteCMD(0x23)
    for i=1,#lut_vcom do
        EPD_W21_WriteDATA(lut_wb[i])
    end
    EPD_W21_WriteCMD(0x24)
    for i=1,#lut_vcom do
        EPD_W21_WriteDATA(lut_bb[i])
    end
    EPD_W21_WriteCMD(0x25)
    for i=1,#lut_vcom do
        EPD_W21_WriteDATA(lut_ww[i])
    end

    EPD_W21_WriteCMD(0x12)--DISPLAY REFRESH
    sys.wait(5)--!!!The delay here is necessary, 200uS at least!!!
    lcd_chkstatus()
end
function EPD_refresh()
    EPD_W21_WriteCMD(0x12)--DISPLAY REFRESH
    sys.wait(5)--!!!The delay here is necessary, 200uS at least!!!
    lcd_chkstatus()
end

function EPD_sleep()
    EPD_W21_WriteCMD(0X50)--  //VCOM AND DATA INTERVAL SETTING
    EPD_W21_WriteDATA(0xf7)-- //WBmode:VBDF 17|D7 VBDW 97 VBDB 57		WBRmode:VBDF F7 VBDW 77 VBDB 37  VBDR B7

    EPD_W21_WriteCMD(0X02)--  	//power off
    lcd_chkstatus()--waiting for the electronic paper IC to release the idle signal
    EPD_W21_WriteCMD(0X07)--  	//deep sleep
    EPD_W21_WriteDATA(0xA5)
end


local clear
sys.taskInit(function ()
    log.info("start! =================")
    EPD_init()
    EPD_display_Clean()
    EPD_refresh()
    EPD_sleep()
    clear = true
    sys.publish("EINK_CLEAR")
end)

require"lbsLoc"
require "http"
lbsLoc.request(function (result,lat,lng)
    log.info("lbsLoc", result,lat,lng)
    if result == 0 and lat and lng then
        nvm.set("lng", lng)
        nvm.set("lat", lat)
    else
        lng = nvm.get("lng")
        lat = nvm.get("lat")
    end
    http.request("POST",
    "https://qq.papapoi.com/eink-calendar/"..misc.getImei().."/"..lng.."/"..lat,
    nil,
    {["Content-Type"]="application/json"},
    json.encode({
        voltage = misc.getVbatt()
    }),
    60000,
    function (result,_,_,body)
        log.info("http request", result,body:len())
        sys.taskInit(function ()
            local img1
            local img2
            if result then
                local timeInfo = body:sub(1,14)
                img1 = body:sub(15,15014)
                img2 = body:sub(15015,30014)

                rtos.make_dir("/last/")
                os.remove("/last/img1")
                os.remove("/last/img2")
                io.writeFile("/last/img1", img1, "wb")
                io.writeFile("/last/img2", img2, "wb")

                body = nil
                collectgarbage("collect")

                local _,year,month,day,hour,min,sec,year_n,month_n,day_n,hour_n,min_n,sec_n =
                pack.unpack(timeInfo,"<HbbbbbHbbbbb")
                log.info("time", year,month,day,hour,min,sec,year_n,month_n,day_n,hour_n,min_n,sec_n)
                misc.setClock({year=year,month=month,day=day,hour=hour,min=min,sec=sec},function ()
                    sys.publish("MISC_SET_CLOCK")
                end)
                sys.waitUntil("MISC_SET_CLOCK",5000)
                rtos.set_alarm(1,year_n,month_n,day_n,hour_n,min_n,sec_n)
            else
                local onTimet = os.date("*t",os.time() + 3600*6)
                rtos.set_alarm(1,onTimet.year,onTimet.month,onTimet.day,onTimet.hour,onTimet.min,onTimet.sec)
                img1 = io.readFile("/last/img1")
                img2 = io.readFile("/last/img2")
            end

            if not clear then--等刷新完
                sys.waitUntil("EINK_CLEAR")
            end
            if img1 and img2 then
                EPD_init_4gray()
                EPD_W21_WriteCMD(0x10)
                dc(1)
                spi.send(spi.SPI_1,img1)
                EPD_W21_WriteCMD(0x13)
                dc(1)
                spi.send(spi.SPI_1,img2)
                EPD_refresh_4gray()
                EPD_sleep()
                -- EPD_init()
                -- EPD_W21_WriteCMD(0x10)--Transfer old data
                -- EPD_W21_WriteDATA_S(white)
                -- EPD_W21_WriteCMD(0x13)--Transfer new data
                -- dc(1)
                -- spi.send(spi.SPI_1,img1)
                -- EPD_refresh()
                -- EPD_sleep()
            end

            sys.wait(10000)
            rtos.poweroff()
        end)
    end)
end)


